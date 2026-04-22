use anchor_lang::prelude::*;

pub const SEED_EMPRESA: &[u8] = b"empresa";
pub const MAX_NOMBRE:    usize = 100;
pub const MAX_RFC:       usize = 13;
pub const MAX_CATEGORIA: usize = 50;
pub const MAX_HASH_DOC:  usize = 64;

#[account]
pub struct Empresa {
    /// ID secuencial asignado por el programa
    pub id: u64,
    /// Wallet del representante
    pub autoridad: Pubkey,
    pub nombre: String,
    pub rfc: String,
    pub categoria: String,
    /// SHA-256 del acta constitutiva (el doc vive off-chain / IPFS)
    pub hash_doc_constitucion: String,
    pub estado: EstadoEmpresa,
    pub registrado_en: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EstadoEmpresa {
    Pendiente,   // esperando activación de membresía
    Activa,
    Suspendida,
}

impl Empresa {
    pub const LEN: usize = 8
        + 8
        + 32
        + (4 + MAX_NOMBRE)
        + (4 + MAX_RFC)
        + (4 + MAX_CATEGORIA)
        + (4 + MAX_HASH_DOC)
        + 1
        + 8
        + 1;
}
