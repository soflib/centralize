use anchor_lang::prelude::*;
use crate::errors::MarketplaceError;
use crate::state::{
    platform::*,
    empresa::{MAX_NOMBRE, MAX_RFC, MAX_CATEGORIA, MAX_HASH_DOC, SEED_EMPRESA},
    proveedor::*,
    membresia::*,
};

// ────────────────────────────────────────────────────────────
// register_proveedor
// ────────────────────────────────────────────────────────────

pub fn register(
    ctx: Context<RegisterProveedor>,
    nombre: String,
    rfc: String,
    descripcion: String,
    categoria: String,
    hash_doc_identidad: String,
    hash_doc_experiencia: String,
) -> Result<()> {
    require!(ctx.accounts.platform.activo, MarketplaceError::PlataformaInactiva);
    require!(!nombre.is_empty() && nombre.len() <= MAX_NOMBRE,       MarketplaceError::NombreInvalido);
    require!(rfc.len() >= 12 && rfc.len() <= MAX_RFC,                MarketplaceError::RfcInvalido);
    require!(hash_doc_identidad.len()   == 64,                       MarketplaceError::HashDocumentoInvalido);
    require!(hash_doc_experiencia.len() == 64,                       MarketplaceError::HashDocumentoInvalido);

    let platform  = &mut ctx.accounts.platform;
    let proveedor = &mut ctx.accounts.proveedor;
    let membresia = &mut ctx.accounts.membresia;
    let clock = Clock::get()?;

    let id = platform.total_proveedores;
    platform.total_proveedores += 1;

    proveedor.id                  = id;
    proveedor.autoridad           = ctx.accounts.autoridad.key();
    proveedor.nombre              = nombre;
    proveedor.rfc                 = rfc.to_uppercase();
    proveedor.descripcion         = descripcion;
    proveedor.categoria           = categoria;
    proveedor.hash_doc_identidad  = hash_doc_identidad;
    proveedor.hash_doc_experiencia = hash_doc_experiencia;
    proveedor.estado              = EstadoProveedor::Registrado;
    proveedor.total_auditorias    = 0;
    proveedor.puntuacion_promedio = 0;
    proveedor.registrado_en       = clock.unix_timestamp;
    proveedor.actualizado_en      = clock.unix_timestamp;
    proveedor.bump                = ctx.bumps.proveedor;

    membresia.titular         = ctx.accounts.autoridad.key();
    membresia.tipo            = TipoMembresia::Proveedor;
    membresia.activa          = false;
    membresia.inicio          = 0;
    membresia.vencimiento     = 0;
    membresia.renovaciones    = 0;
    membresia.hash_ultimo_pago = String::new();
    membresia.referencia_pago  = String::new();
    membresia.ultimo_pago_en  = 0;
    membresia.bump            = ctx.bumps.membresia;

    msg!("Proveedor registrado: {} | ID: {}", proveedor.nombre, proveedor.id);
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterProveedor<'info> {
    #[account(
        mut,
        seeds = [SEED_PLATFORM],
        bump  = platform.bump,
    )]
    pub platform: Account<'info, PlatformConfig>,

    #[account(
        init,
        payer = autoridad,
        space = Proveedor::LEN,
        seeds = [SEED_PROVEEDOR, &platform.total_proveedores.to_le_bytes()],
        bump
    )]
    pub proveedor: Account<'info, Proveedor>,

    #[account(
        init,
        payer = autoridad,
        space = Membresia::LEN,
        seeds = [SEED_MEMBRESIA, autoridad.key().as_ref()],
        bump
    )]
    pub membresia: Account<'info, Membresia>,

    #[account(mut)]
    pub autoridad: Signer<'info>,

    pub system_program: Program<'info, System>,
}
