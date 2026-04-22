use anchor_lang::prelude::*;
use crate::errors::MarketplaceError;
use crate::state::{
    platform::*,
    empresa::*,
    membresia::*,
};

// ────────────────────────────────────────────────────────────
// register_empresa
// Cualquier wallet puede registrar una empresa.
// La membresía queda en estado inactivo hasta que el admin
// confirme el pago (llamando a activar_membresia).
// ────────────────────────────────────────────────────────────

pub fn register(
    ctx: Context<RegisterEmpresa>,
    nombre: String,
    rfc: String,
    categoria: String,
    hash_doc_constitucion: String,
) -> Result<()> {
    // Validaciones básicas
    require!(ctx.accounts.platform.activo, MarketplaceError::PlataformaInactiva);
    require!(!nombre.is_empty() && nombre.len() <= MAX_NOMBRE, MarketplaceError::NombreInvalido);
    require!(rfc.len() >= 12 && rfc.len() <= MAX_RFC,          MarketplaceError::RfcInvalido);
    require!(hash_doc_constitucion.len() == 64,                MarketplaceError::HashDocumentoInvalido);

    let platform = &mut ctx.accounts.platform;
    let empresa  = &mut ctx.accounts.empresa;
    let membresia = &mut ctx.accounts.membresia;
    let clock = Clock::get()?;

    // Asignar ID secuencial
    let id = platform.total_empresas;
    platform.total_empresas += 1;

    // Poblar la cuenta Empresa
    empresa.id                    = id;
    empresa.autoridad             = ctx.accounts.autoridad.key();
    empresa.nombre                = nombre;
    empresa.rfc                   = rfc.to_uppercase();
    empresa.categoria             = categoria;
    empresa.hash_doc_constitucion = hash_doc_constitucion;
    empresa.estado                = EstadoEmpresa::Pendiente;
    empresa.registrado_en         = clock.unix_timestamp;
    empresa.bump                  = ctx.bumps.empresa;

    // Crear membresía vacía (inactiva hasta confirmar pago)
    membresia.titular         = ctx.accounts.autoridad.key();
    membresia.tipo            = TipoMembresia::Empresa;
    membresia.activa          = false;
    membresia.inicio          = 0;
    membresia.vencimiento     = 0;
    membresia.renovaciones    = 0;
    membresia.hash_ultimo_pago = String::new();
    membresia.referencia_pago  = String::new();
    membresia.ultimo_pago_en  = 0;
    membresia.bump            = ctx.bumps.membresia;

    msg!("Empresa registrada: {} | ID: {} | RFC: {}", empresa.nombre, empresa.id, empresa.rfc);
    msg!("Membresía pendiente de activación.");
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterEmpresa<'info> {
    #[account(
        mut,
        seeds = [SEED_PLATFORM],
        bump  = platform.bump,
    )]
    pub platform: Account<'info, PlatformConfig>,

    #[account(
        init,
        payer = autoridad,
        space = Empresa::LEN,
        seeds = [SEED_EMPRESA, &platform.total_empresas.to_le_bytes()],
        bump
    )]
    pub empresa: Account<'info, Empresa>,

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
