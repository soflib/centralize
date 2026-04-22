use anchor_lang::prelude::*;

pub const SEED_PLATFORM: &[u8] = b"platform";

#[account]
pub struct PlatformConfig {
    /// Wallet del administrador (tu wallet)
    pub admin: Pubkey,
    /// El programa acepta nuevos registros
    pub activo: bool,
    /// Versión del contrato
    pub version: u8,
    /// Contadores globales
    pub total_empresas: u64,
    pub total_proveedores: u64,
    pub total_auditorias: u64,
    /// Timestamp de creación
    pub creado_en: i64,
    pub bump: u8,
}

impl PlatformConfig {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 8 + 8 + 8 + 8 + 1;
}
