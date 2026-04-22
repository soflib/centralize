use anchor_lang::prelude::*;

pub const SEED_MEMBRESIA: &[u8] = b"membresia";
pub const DURACION_ANUAL: i64   = 365 * 24 * 60 * 60;

#[account]
pub struct Membresia {
    pub titular: Pubkey,
    pub tipo: TipoMembresia,
    pub activa: bool,
    pub inicio: i64,
    pub vencimiento: i64,
    pub renovaciones: u32,
    /// SHA-256 del comprobante de pago (off-chain)
    pub hash_ultimo_pago: String,
    /// ID de transacción de MercadoPago / Stripe
    pub referencia_pago: String,
    pub ultimo_pago_en: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TipoMembresia {
    Empresa,    // 300 MXN / año
    Proveedor,  // 100 MXN / año
}

impl Membresia {
    pub const LEN: usize = 8
        + 32
        + 1
        + 1
        + 8
        + 8
        + 4
        + (4 + crate::state::empresa::MAX_HASH_DOC)
        + (4 + crate::state::empresa::MAX_HASH_DOC)
        + 8
        + 1;
}
