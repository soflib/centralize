// ============================================================
//  db.rs — PostgreSQL con tokio-postgres + deadpool-postgres
// ============================================================

use anyhow::Result;
use deadpool_postgres::Pool;
use crate::models::{Event, Section, Ticket};

// ── Inicializar tablas ───────────────────────────────────────
pub async fn init(pool: &Pool) -> Result<()> {
    let client = pool.get().await?;

    client.execute(r#"
        CREATE TABLE IF NOT EXISTS events (
            id       TEXT PRIMARY KEY,
            name     TEXT NOT NULL,
            venue    TEXT NOT NULL,
            date     TEXT NOT NULL,
            time     TEXT NOT NULL,
            price    TEXT NOT NULL,
            image    TEXT NOT NULL,
            capacity INT  NOT NULL,
            sold     INT  NOT NULL DEFAULT 0
        )
    "#, &[]).await?;

    client.execute(r#"
        CREATE TABLE IF NOT EXISTS sections (
            id        TEXT NOT NULL,
            event_id  TEXT NOT NULL REFERENCES events(id),
            name      TEXT NOT NULL,
            price     TEXT NOT NULL,
            color     TEXT NOT NULL,
            available INT  NOT NULL,
            PRIMARY KEY (id, event_id)
        )
    "#, &[]).await?;

    client.execute(r#"
        CREATE TABLE IF NOT EXISTS tickets (
            id            TEXT PRIMARY KEY,
            number        SERIAL,
            event_id      TEXT NOT NULL,
            event_name    TEXT NOT NULL,
            venue         TEXT NOT NULL,
            date          TEXT NOT NULL,
            time          TEXT NOT NULL,
            section       TEXT NOT NULL,
            section_id    TEXT NOT NULL,
            section_color TEXT NOT NULL,
            price         TEXT NOT NULL,
            buyer_name    TEXT NOT NULL,
            buyer_email   TEXT NOT NULL,
            tx_signature  TEXT NOT NULL,
            purchased_at  TEXT NOT NULL,
            is_used       BOOLEAN NOT NULL DEFAULT false,
            used_at       TEXT,
            validate_tx   TEXT
        )
    "#, &[]).await?;

    // Insertar evento por defecto si no existe
    let row = client.query_one(
        "SELECT EXISTS(SELECT 1 FROM events WHERE id = 'EVT001')",
        &[],
    ).await?;
    let exists: bool = row.get(0);

    if !exists {
        seed_default_event(pool).await?;
    }

    Ok(())
}

// ── Seed evento por defecto ──────────────────────────────────
async fn seed_default_event(pool: &Pool) -> Result<()> {
    let client = pool.get().await?;

    client.execute(
        "INSERT INTO events (id, name, venue, date, time, price, image, capacity, sold)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
        &[
            &"EVT001",
            &"Noche Estelar — Festival de Música",
            &"Arena Ciudad de México",
            &"2026-06-15",
            &"20:00",
            &"$850 MXN",
            &"🎵",
            &500_i32,
            &0_i32,
        ],
    ).await?;

    let sections: Vec<(&str, &str, &str, &str, i32)> = vec![
        ("VIP",     "VIP",     "$1,200 MXN", "#FFD700", 50),
        ("GENERAL", "General", "$850 MXN",   "#A78BFA", 300),
        ("PISTA",   "Pista",   "$650 MXN",   "#34D399", 150),
    ];

    for (id, name, price, color, available) in sections {
        client.execute(
            "INSERT INTO sections (id, event_id, name, price, color, available)
             VALUES ($1,$2,$3,$4,$5,$6)",
            &[&id, &"EVT001", &name, &price, &color, &available],
        ).await?;
    }

    Ok(())
}

// ── get_event ────────────────────────────────────────────────
pub async fn get_event(pool: &Pool) -> Result<Option<Event>> {
    let client = pool.get().await?;

    let rows = client.query(
        "SELECT id, name, venue, date, time, price, image, capacity, sold
         FROM events WHERE id = 'EVT001'",
        &[],
    ).await?;

    let Some(row) = rows.into_iter().next() else {
        return Ok(None);
    };

    let event_id: String = row.get("id");

    let sec_rows = client.query(
        "SELECT id, name, price, color, available FROM sections WHERE event_id = $1",
        &[&event_id],
    ).await?;

    let sections = sec_rows.iter().map(|s| Section {
        id:        s.get("id"),
        name:      s.get("name"),
        price:     s.get("price"),
        color:     s.get("color"),
        available: (s.get::<_, i32>("available")) as u32,
    }).collect();

    Ok(Some(Event {
        id:       event_id,
        name:     row.get("name"),
        venue:    row.get("venue"),
        date:     row.get("date"),
        time:     row.get("time"),
        price:    row.get("price"),
        image:    row.get("image"),
        capacity: (row.get::<_, i32>("capacity")) as u32,
        sold:     (row.get::<_, i32>("sold")) as u32,
        sections,
    }))
}

// ── get_ticket ───────────────────────────────────────────────
pub async fn get_ticket(pool: &Pool, id: &str) -> Result<Option<Ticket>> {
    let client = pool.get().await?;

    let rows = client.query(
        "SELECT * FROM tickets WHERE id = $1",
        &[&id],
    ).await?;

    Ok(rows.into_iter().next().map(|r| ticket_from_row(&r)))
}

// ── list_tickets ─────────────────────────────────────────────
pub async fn list_tickets(pool: &Pool) -> Result<Vec<Ticket>> {
    let client = pool.get().await?;

    let rows = client.query(
        "SELECT * FROM tickets ORDER BY number ASC",
        &[],
    ).await?;

    Ok(rows.iter().map(ticket_from_row).collect())
}

// ── insert_ticket ────────────────────────────────────────────
pub async fn insert_ticket(pool: &Pool, t: &Ticket) -> Result<()> {
    let client = pool.get().await?;

    client.execute(
        r#"INSERT INTO tickets
           (id, event_id, event_name, venue, date, time, section,
            section_id, section_color, price, buyer_name, buyer_email,
            tx_signature, purchased_at, is_used, used_at, validate_tx)
         VALUES
           ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17)"#,
        &[
            &t.id, &t.event_id, &t.event_name,
            &t.venue, &t.date, &t.time,
            &t.section, &t.section_id, &t.section_color,
            &t.price, &t.buyer_name, &t.buyer_email,
            &t.tx_signature, &t.purchased_at,
            &t.is_used, &t.used_at, &t.validate_tx,
        ],
    ).await?;

    Ok(())
}

// ── decrement_section ────────────────────────────────────────
pub async fn decrement_section(pool: &Pool, event_id: &str, section_id: &str) -> Result<()> {
    let client = pool.get().await?;

    client.execute(
        "UPDATE sections SET available = GREATEST(0, available - 1)
         WHERE event_id = $1 AND id = $2",
        &[&event_id, &section_id],
    ).await?;

    client.execute(
        "UPDATE events SET sold = sold + 1 WHERE id = $1",
        &[&event_id],
    ).await?;

    Ok(())
}

// ── mark_used ────────────────────────────────────────────────
pub async fn mark_used(pool: &Pool, id: &str, used_at: &str, validate_tx: &str) -> Result<()> {
    let client = pool.get().await?;

    client.execute(
        "UPDATE tickets SET is_used = true, used_at = $1, validate_tx = $2 WHERE id = $3",
        &[&used_at, &validate_tx, &id],
    ).await?;

    Ok(())
}

// ── Helper: construir Ticket desde una Row ────────────────────
fn ticket_from_row(r: &tokio_postgres::Row) -> Ticket {
    Ticket {
        id:            r.get("id"),
        number:        (r.get::<_, i32>("number")) as u32,
        event_id:      r.get("event_id"),
        event_name:    r.get("event_name"),
        venue:         r.get("venue"),
        date:          r.get("date"),
        time:          r.get("time"),
        section:       r.get("section"),
        section_id:    r.get("section_id"),
        section_color: r.get("section_color"),
        price:         r.get("price"),
        buyer_name:    r.get("buyer_name"),
        buyer_email:   r.get("buyer_email"),
        tx_signature:  r.get("tx_signature"),
        purchased_at:  r.get("purchased_at"),
        is_used:       r.get("is_used"),
        used_at:       r.get("used_at"),
        validate_tx:   r.get("validate_tx"),
    }
}