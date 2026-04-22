use anchor_lang::prelude::*;

pub const SEED_AUDITORIA: &[u8] = b"auditoria";
pub const MAX_COMENTARIO: usize = 300;

#[account]
pub struct Auditoria {
    pub empresa: Pubkey,
    pub proveedor: Pubkey,
    pub estado: EstadoAuditoria,
    /// SHA-256 del reporte completo (off-chain)
    pub hash_reporte: String,
    /// Comentario público que queda en cadena para siempre
    pub comentario: String,
    pub resultado: ResultadoAuditoria,
    pub iniciado_en: i64,
    /// 0 mientras no concluye
    pub concluido_en: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EstadoAuditoria {
    Iniciada,
    EnProceso,
    Concluida,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ResultadoAuditoria {
    Pendiente,
    Aprobado,
    Rechazado,
    Observaciones,
}

impl Auditoria {
    pub const LEN: usize = 8
        + 32
        + 32
        + 1
        + (4 + crate::state::empresa::MAX_HASH_DOC)
        + (4 + MAX_COMENTARIO)
        + 1
        + 8
        + 8
        + 1;
}
