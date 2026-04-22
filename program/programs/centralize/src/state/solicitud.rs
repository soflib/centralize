use anchor_lang::prelude::*;

pub const SEED_SOLICITUD: &[u8] = b"solicitud";

#[account]
pub struct SolicitudServicio {
    pub id: u64,
    pub empresa: Pubkey,
    pub categoria: String,
    /// SHA-256 del brief completo (off-chain, solo visible para proveedores verificados)
    pub hash_detalle: String,
    /// Descripción corta visible on-chain
    pub descripcion_breve: String,
    pub estado: EstadoSolicitud,
    /// None hasta que se asigne
    pub proveedor_asignado: Option<Pubkey>,
    pub fecha_limite: i64,
    pub publicado_en: i64,
    pub cerrado_en: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EstadoSolicitud {
    Abierta,
    EnEvaluacion,
    Asignada,
    Completada,
    Cancelada,
}

impl SolicitudServicio {
    pub const LEN: usize = 8
        + 8
        + 32
        + (4 + crate::state::empresa::MAX_CATEGORIA)
        + (4 + crate::state::empresa::MAX_HASH_DOC)
        + (4 + crate::state::proveedor::MAX_DESCRIPCION)
        + 1
        + (1 + 32)   // Option<Pubkey>
        + 8
        + 8
        + 8
        + 1;
}
