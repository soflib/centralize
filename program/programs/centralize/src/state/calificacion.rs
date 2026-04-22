use anchor_lang::prelude::*;

pub const SEED_CALIFICACION: &[u8] = b"calificacion";

#[account]
pub struct Calificacion {
    pub empresa: Pubkey,
    pub proveedor: Pubkey,
    /// Puntuaciones 1-10
    pub puntuacion:             u8,
    pub puntuacion_cumplimiento: u8,
    pub puntuacion_calidad:      u8,
    pub puntuacion_precio:       u8,
    pub comentario: String,
    pub recomienda: bool,
    pub creado_en: i64,
    pub bump: u8,
}

impl Calificacion {
    pub const LEN: usize = 8
        + 32
        + 32
        + 1 + 1 + 1 + 1
        + (4 + crate::state::auditoria::MAX_COMENTARIO)
        + 1
        + 8
        + 1;
}
