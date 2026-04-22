use anchor_lang::prelude::*;

pub const SEED_PROVEEDOR:   &[u8] = b"proveedor";
pub const MAX_DESCRIPCION:  usize = 500;

#[account]
pub struct Proveedor {
    pub id: u64,
    pub autoridad: Pubkey,
    pub nombre: String,
    pub rfc: String,
    pub descripcion: String,
    pub categoria: String,
    /// SHA-256 del INE / pasaporte (off-chain)
    pub hash_doc_identidad: String,
    /// SHA-256 del portafolio / cartas de recomendación (off-chain)
    pub hash_doc_experiencia: String,
    pub estado: EstadoProveedor,
    pub total_auditorias: u32,
    /// Promedio ponderado 0-100  →  divide entre 10 para mostrar "8.5"
    pub puntuacion_promedio: u8,
    pub registrado_en: i64,
    pub actualizado_en: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EstadoProveedor {
    Registrado,  // sin auditoría aún
    EnRevision,  // al menos una empresa lo audita
    Verificado,  // aprobado
    Rechazado,
    Suspendido,
}

impl Proveedor {
    pub const LEN: usize = 8
        + 8
        + 32
        + (4 + crate::state::empresa::MAX_NOMBRE)
        + (4 + crate::state::empresa::MAX_RFC)
        + (4 + MAX_DESCRIPCION)
        + (4 + crate::state::empresa::MAX_CATEGORIA)
        + (4 + crate::state::empresa::MAX_HASH_DOC)
        + (4 + crate::state::empresa::MAX_HASH_DOC)
        + 1
        + 4
        + 1
        + 8
        + 8
        + 1;
}
