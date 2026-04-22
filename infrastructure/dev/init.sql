mkdir -p sql
cat > sql/init.sql << 'EOF'
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS issuers (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wallet_pubkey   VARCHAR(44) UNIQUE NOT NULL,
    name            VARCHAR(64) NOT NULL,
    issuer_type     VARCHAR(32) NOT NULL,
    country_code    CHAR(2) NOT NULL,
    website_uri     VARCHAR(200),
    email           VARCHAR(255),
    is_active       BOOLEAN DEFAULT true,
    created_at      TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS certificates (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    cert_id          UUID UNIQUE NOT NULL,
    issuer_id        UUID REFERENCES issuers(id),
    recipient_wallet VARCHAR(44) NOT NULL,
    recipient_name   VARCHAR(64) NOT NULL,
    recipient_email  VARCHAR(255),
    cert_type        VARCHAR(32) NOT NULL,
    program_name     VARCHAR(64) NOT NULL,
    grade            NUMERIC(5,2),
    issue_date       TIMESTAMP NOT NULL,
    expiry_date      TIMESTAMP,
    metadata_uri     VARCHAR(200),
    metadata_hash    CHAR(64),
    is_revoked       BOOLEAN DEFAULT false,
    solana_tx_hash   VARCHAR(88),
    created_at       TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS events (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id         UUID UNIQUE NOT NULL,
    organizer_wallet VARCHAR(44) NOT NULL,
    name             VARCHAR(64) NOT NULL,
    description      TEXT,
    venue            VARCHAR(64),
    start_time       TIMESTAMP NOT NULL,
    end_time         TIMESTAMP NOT NULL,
    total_capacity   INTEGER NOT NULL,
    sold_count       INTEGER DEFAULT 0,
    price_lamports   BIGINT NOT NULL,
    is_active        BOOLEAN DEFAULT true,
    solana_tx_hash   VARCHAR(88),
    created_at       TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tickets (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id        UUID REFERENCES events(id),
    ticket_index    INTEGER NOT NULL,
    owner_wallet    VARCHAR(44) NOT NULL,
    owner_email     VARCHAR(255),
    seat_info       VARCHAR(64),
    purchased_at    TIMESTAMP DEFAULT NOW(),
    paid_lamports   BIGINT,
    is_used         BOOLEAN DEFAULT false,
    used_at         TIMESTAMP,
    is_listed       BOOLEAN DEFAULT false,
    solana_tx_hash  VARCHAR(88),
    UNIQUE(event_id, ticket_index)
);
EOF