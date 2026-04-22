use anchor_lang::prelude::*;
use crate::errors::MarketplaceError;
use crate::state::{
    platform::*,
    empresa::{Empresa, EstadoEmpresa, SEED_EMPRESA},
    proveedor::{Proveedor, EstadoProveedor, SEED_PROVEEDOR},
    membresia::{Membresia, SEED_MEMBRESIA},
    solicitud::*,
};

// ────────────────────────────────────────────────────────────
// publicar_solicitud
// Empresa con membresía activa publica una solicitud privada.
// El detalle completo vive off-chain; solo el hash llega aquí.
// ────────────────────────────────────────────────────────────

pub fn publicar(
    ctx: Context<PublicarSolicitud>,
    categoria: String,
    hash_detalle: String,
    descripcion_breve: String,
    fecha_limite: i64,
) -> Result<()> {
    let clock = Clock::get()?;

    require!(
        ctx.accounts.membresia_empresa.activa &&
        ctx.accounts.membresia_empresa.vencimiento > clock.unix_timestamp,
        MarketplaceError::MembresiaInactiva
    );
    require!(
        ctx.accounts.empresa.estado == EstadoEmpresa::Activa,
        MarketplaceError::EmpresaNoActiva
    );
    require!(fecha_limite > clock.unix_timestamp, MarketplaceError::FechaLimiteInvalida);
    require!(hash_detalle.len() == 64,            MarketplaceError::HashDocumentoInvalido);
    require!(
        !descripcion_breve.is_empty() &&
        descripcion_breve.len() <= crate::state::proveedor::MAX_DESCRIPCION,
        MarketplaceError::DescripcionMuyLarga
    );

    let platform  = &mut ctx.accounts.platform;
    let solicitud = &mut ctx.accounts.solicitud;

    // Usamos total_auditorias como contador reutilizable para el ID de solicitud
    // En producción puedes agregar un campo total_solicitudes al platform
    let id = platform.total_auditorias;

    solicitud.id                = id;
    solicitud.empresa           = ctx.accounts.empresa.key();
    solicitud.categoria         = categoria;
    solicitud.hash_detalle      = hash_detalle;
    solicitud.descripcion_breve = descripcion_breve;
    solicitud.estado            = EstadoSolicitud::Abierta;
    solicitud.proveedor_asignado = None;
    solicitud.fecha_limite      = fecha_limite;
    solicitud.publicado_en      = clock.unix_timestamp;
    solicitud.cerrado_en        = 0;
    solicitud.bump              = ctx.bumps.solicitud;

    msg!(
        "Solicitud publicada | ID: {} | Empresa: {} | Categoría: {}",
        solicitud.id,
        solicitud.empresa,
        solicitud.categoria
    );
    Ok(())
}

// ────────────────────────────────────────────────────────────
// asignar_solicitud
// La empresa elige a un proveedor verificado con membresía activa.
// ────────────────────────────────────────────────────────────

pub fn asignar(ctx: Context<AsignarSolicitud>) -> Result<()> {
    let clock = Clock::get()?;

    require!(
        ctx.accounts.solicitud.estado == EstadoSolicitud::Abierta ||
        ctx.accounts.solicitud.estado == EstadoSolicitud::EnEvaluacion,
        MarketplaceError::SolicitudNoAbierta
    );
    require!(
        ctx.accounts.solicitud.empresa == ctx.accounts.empresa.key(),
        MarketplaceError::NoEsDuenoDeSolicitud
    );
    require!(
        ctx.accounts.empresa.autoridad == ctx.accounts.empresa_autoridad.key(),
        MarketplaceError::NoEsDuenoDeSolicitud
    );

    // Proveedor debe estar verificado
    require!(
        ctx.accounts.proveedor.estado == EstadoProveedor::Verificado,
        MarketplaceError::ProveedorNoVerificado
    );

    // Proveedor debe tener membresía activa
    require!(
        ctx.accounts.membresia_proveedor.activa &&
        ctx.accounts.membresia_proveedor.vencimiento > clock.unix_timestamp,
        MarketplaceError::MembresiaInactiva
    );

    let solicitud = &mut ctx.accounts.solicitud;

    solicitud.estado             = EstadoSolicitud::Asignada;
    solicitud.proveedor_asignado = Some(ctx.accounts.proveedor.key());

    msg!(
        "Solicitud {} asignada a proveedor: {}",
        solicitud.id,
        ctx.accounts.proveedor.key()
    );
    Ok(())
}

// ────────────────────────────────────────────────────────────
// cerrar_solicitud
// La empresa marca el servicio como completado o cancelado.
// ────────────────────────────────────────────────────────────

pub fn cerrar(
    ctx: Context<CerrarSolicitud>,
    completada: bool,
) -> Result<()> {
    let clock = Clock::get()?;

    require!(
        ctx.accounts.solicitud.estado != EstadoSolicitud::Completada &&
        ctx.accounts.solicitud.estado != EstadoSolicitud::Cancelada,
        MarketplaceError::SolicitudCerrada
    );
    require!(
        ctx.accounts.solicitud.empresa == ctx.accounts.empresa.key(),
        MarketplaceError::NoEsDuenoDeSolicitud
    );
    require!(
        ctx.accounts.empresa.autoridad == ctx.accounts.empresa_autoridad.key(),
        MarketplaceError::NoEsDuenoDeSolicitud
    );

    let solicitud = &mut ctx.accounts.solicitud;

    solicitud.estado     = if completada {
        EstadoSolicitud::Completada
    } else {
        EstadoSolicitud::Cancelada
    };
    solicitud.cerrado_en = clock.unix_timestamp;

    msg!(
        "Solicitud {} cerrada como: {}",
        solicitud.id,
        if completada { "Completada" } else { "Cancelada" }
    );
    Ok(())
}

// ────────────────────────────────────────────────────────────
// Contextos
// ────────────────────────────────────────────────────────────

#[derive(Accounts)]
pub struct PublicarSolicitud<'info> {
    #[account(mut, seeds = [SEED_PLATFORM], bump = platform.bump)]
    pub platform: Account<'info, PlatformConfig>,

    #[account(
        seeds = [SEED_EMPRESA, &empresa.id.to_le_bytes()],
        bump  = empresa.bump,
        constraint = empresa.autoridad == empresa_autoridad.key()
    )]
    pub empresa: Account<'info, Empresa>,

    #[account(
        seeds = [SEED_MEMBRESIA, empresa.autoridad.as_ref()],
        bump  = membresia_empresa.bump,
    )]
    pub membresia_empresa: Account<'info, Membresia>,

    #[account(
        init,
        payer = empresa_autoridad,
        space = SolicitudServicio::LEN,
        seeds = [SEED_SOLICITUD, empresa.key().as_ref(), &platform.total_auditorias.to_le_bytes()],
        bump
    )]
    pub solicitud: Account<'info, SolicitudServicio>,

    #[account(mut)]
    pub empresa_autoridad: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AsignarSolicitud<'info> {
    #[account(
        seeds = [SEED_EMPRESA, &empresa.id.to_le_bytes()],
        bump  = empresa.bump,
    )]
    pub empresa: Account<'info, Empresa>,

    #[account(
        seeds = [SEED_PROVEEDOR, &proveedor.id.to_le_bytes()],
        bump  = proveedor.bump,
    )]
    pub proveedor: Account<'info, Proveedor>,

    #[account(
        seeds = [SEED_MEMBRESIA, proveedor.autoridad.as_ref()],
        bump  = membresia_proveedor.bump,
    )]
    pub membresia_proveedor: Account<'info, Membresia>,

    #[account(
        mut,
        seeds = [SEED_SOLICITUD, empresa.key().as_ref(), &solicitud.id.to_le_bytes()],
        bump  = solicitud.bump,
    )]
    pub solicitud: Account<'info, SolicitudServicio>,

    pub empresa_autoridad: Signer<'info>,
}

#[derive(Accounts)]
pub struct CerrarSolicitud<'info> {
    #[account(
        seeds = [SEED_EMPRESA, &empresa.id.to_le_bytes()],
        bump  = empresa.bump,
    )]
    pub empresa: Account<'info, Empresa>,

    #[account(
        mut,
        seeds = [SEED_SOLICITUD, empresa.key().as_ref(), &solicitud.id.to_le_bytes()],
        bump  = solicitud.bump,
    )]
    pub solicitud: Account<'info, SolicitudServicio>,

    pub empresa_autoridad: Signer<'info>,
}
