use anchor_lang::prelude::*;
use crate::errors::MarketplaceError;
use crate::state::{
    platform::*,
    empresa::{Empresa, EstadoEmpresa, MAX_HASH_DOC, SEED_EMPRESA},
    proveedor::{Proveedor, EstadoProveedor, SEED_PROVEEDOR},
    membresia::*,
};

// ────────────────────────────────────────────────────────────
// activar_membresia_empresa
// Solo el admin la llama después de confirmar el pago fiat.
// Activa la membresía y marca la empresa como Activa.
// ────────────────────────────────────────────────────────────

pub fn activar_empresa(
    ctx: Context<ActivarEmpresa>,
    hash_comprobante: String,
    referencia_pago: String,
) -> Result<()> {
    require!(
        ctx.accounts.platform.admin == ctx.accounts.admin.key(),
        MarketplaceError::NoEsAdmin
    );
    require!(referencia_pago.len() > 0, MarketplaceError::ReferenciaPagoInvalida);
    require!(hash_comprobante.len() == 64, MarketplaceError::HashDocumentoInvalido);
    require!(!ctx.accounts.membresia.activa, MarketplaceError::MembresiaYaActiva);

    let clock = Clock::get()?;
    let membresia = &mut ctx.accounts.membresia;
    let empresa   = &mut ctx.accounts.empresa;

    membresia.activa           = true;
    membresia.inicio           = clock.unix_timestamp;
    membresia.vencimiento      = clock.unix_timestamp + DURACION_ANUAL;
    membresia.hash_ultimo_pago = hash_comprobante;
    membresia.referencia_pago  = referencia_pago;
    membresia.ultimo_pago_en   = clock.unix_timestamp;

    empresa.estado = EstadoEmpresa::Activa;

    msg!(
        "Membresía empresa activada. Vence: {} | Ref: {}",
        membresia.vencimiento,
        membresia.referencia_pago
    );
    Ok(())
}

// ────────────────────────────────────────────────────────────
// activar_membresia_proveedor
// ────────────────────────────────────────────────────────────

pub fn activar_proveedor(
    ctx: Context<ActivarProveedor>,
    hash_comprobante: String,
    referencia_pago: String,
) -> Result<()> {
    require!(
        ctx.accounts.platform.admin == ctx.accounts.admin.key(),
        MarketplaceError::NoEsAdmin
    );
    require!(referencia_pago.len() > 0,   MarketplaceError::ReferenciaPagoInvalida);
    require!(hash_comprobante.len() == 64, MarketplaceError::HashDocumentoInvalido);
    require!(!ctx.accounts.membresia.activa, MarketplaceError::MembresiaYaActiva);

    let clock = Clock::get()?;
    let membresia  = &mut ctx.accounts.membresia;

    membresia.activa           = true;
    membresia.inicio           = clock.unix_timestamp;
    membresia.vencimiento      = clock.unix_timestamp + DURACION_ANUAL;
    membresia.hash_ultimo_pago = hash_comprobante;
    membresia.referencia_pago  = referencia_pago;
    membresia.ultimo_pago_en   = clock.unix_timestamp;

    msg!("Membresía proveedor activada. Vence: {}", membresia.vencimiento);
    Ok(())
}

// ────────────────────────────────────────────────────────────
// renovar_membresia
// Sirve para ambos tipos. Extiende desde la fecha de vencimiento
// actual (no desde hoy) para no perder días si renueva tarde.
// ────────────────────────────────────────────────────────────

pub fn renovar(
    ctx: Context<RenovarMembresia>,
    hash_comprobante: String,
    referencia_pago: String,
) -> Result<()> {
    require!(
        ctx.accounts.platform.admin == ctx.accounts.admin.key(),
        MarketplaceError::NoEsAdmin
    );
    require!(referencia_pago.len() > 0,   MarketplaceError::ReferenciaPagoInvalida);
    require!(hash_comprobante.len() == 64, MarketplaceError::HashDocumentoInvalido);

    let clock = Clock::get()?;
    let membresia = &mut ctx.accounts.membresia;

    // Si ya venció, renovar desde hoy. Si aún vigente, extender desde el vencimiento.
    let base = if membresia.vencimiento > clock.unix_timestamp {
        membresia.vencimiento
    } else {
        clock.unix_timestamp
    };

    membresia.activa           = true;
    membresia.vencimiento      = base + DURACION_ANUAL;
    membresia.renovaciones    += 1;
    membresia.hash_ultimo_pago = hash_comprobante;
    membresia.referencia_pago  = referencia_pago;
    membresia.ultimo_pago_en   = clock.unix_timestamp;

    msg!(
        "Membresía renovada #{} | Nuevo vencimiento: {}",
        membresia.renovaciones,
        membresia.vencimiento
    );
    Ok(())
}

// ────────────────────────────────────────────────────────────
// Contextos
// ────────────────────────────────────────────────────────────

#[derive(Accounts)]
pub struct ActivarEmpresa<'info> {
    #[account(seeds = [SEED_PLATFORM], bump = platform.bump)]
    pub platform: Account<'info, PlatformConfig>,

    #[account(
        mut,
        seeds = [SEED_EMPRESA, &empresa.id.to_le_bytes()],
        bump  = empresa.bump,
        constraint = empresa.autoridad == membresia.titular
    )]
    pub empresa: Account<'info, Empresa>,

    #[account(
        mut,
        seeds = [SEED_MEMBRESIA, empresa.autoridad.as_ref()],
        bump  = membresia.bump,
    )]
    pub membresia: Account<'info, Membresia>,

    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct ActivarProveedor<'info> {
    #[account(seeds = [SEED_PLATFORM], bump = platform.bump)]
    pub platform: Account<'info, PlatformConfig>,

    #[account(
        seeds = [SEED_PROVEEDOR, &proveedor.id.to_le_bytes()],
        bump  = proveedor.bump,
    )]
    pub proveedor: Account<'info, Proveedor>,

    #[account(
        mut,
        seeds = [SEED_MEMBRESIA, proveedor.autoridad.as_ref()],
        bump  = membresia.bump,
    )]
    pub membresia: Account<'info, Membresia>,

    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct RenovarMembresia<'info> {
    #[account(seeds = [SEED_PLATFORM], bump = platform.bump)]
    pub platform: Account<'info, PlatformConfig>,

    #[account(mut)]
    pub membresia: Account<'info, Membresia>,

    pub admin: Signer<'info>,
}
