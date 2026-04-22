use serde::{Deserialize, Serialize};
use deadpool_postgres::Pool;
use crate::solana::solana::SolanaClient;

// ── AppState ─────────────────────────────────────────────────
pub struct AppState {
    pub pool:   Pool,
    pub solana: SolanaClient,
}

// ── Modelos de dominio ────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id:        String,
    pub name:      String,
    pub price:     String,
    pub color:     String,
    pub available: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id:       String,
    pub name:     String,
    pub venue:    String,
    pub date:     String,
    pub time:     String,
    pub price:    String,
    pub image:    String,
    pub capacity: u32,
    pub sold:     u32,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id:            String,
    pub number:        u32,
    pub event_id:      String,
    pub event_name:    String,
    pub venue:         String,
    pub date:          String,
    pub time:          String,
    pub section:       String,
    pub section_id:    String,
    pub section_color: String,
    pub price:         String,
    pub buyer_name:    String,
    pub buyer_email:   String,
    pub tx_signature:  String,
    pub purchased_at:  String,
    pub is_used:       bool,
    pub used_at:       Option<String>,
    pub validate_tx:   Option<String>,
}

// ── Request / Response DTOs ───────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct PurchaseRequest {
    pub name:    String,
    pub email:   String,
    pub section: String,
}

#[derive(Debug, Serialize)]
pub struct PurchaseResponse {
    pub success: bool,
    pub ticket:  Ticket,
}

#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub ticket_id:    String,
    pub tx_signature: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidateResponse {
    pub valid:   bool,
    pub message: String,
    pub ticket:  Option<Ticket>,
    pub used_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}