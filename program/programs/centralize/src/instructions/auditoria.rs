use anchor_lang::prelude::*;
use crate::errors::MarketplaceError;
use crate::state::{
    platform::*,
    empresa::{Empresa, EstadoEmpresa, MAX_HASH_DOC, SEED_EMPRESA},
    proveedor::{Proveedor, EstadoProveedor, SEED_PROVEEDOR},
    membresia::{Membresia, SEED_MEMBRESIA},
    auditoria::*,
};

pub fn iniciar(ctx: Context<IniciarAuditoria>) -> Result<()> {
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
    require!(
        ctx.accounts.proveedor.estado != EstadoProveedor::Suspendido,
        MarketplaceError::ProveedorSuspendido
    );

    let empresa_key   = ctx.accounts.empresa.key();
    let proveedor_key = ctx.accounts.proveedor.key();

    let auditoria = &mut ctx.accounts.auditoria;
    auditoria.empresa      = empresa_key;
    auditoria.proveedor    = proveedor_key;
    auditoria.estado       = EstadoAuditoria::Iniciada;
    auditoria.hash_reporte = String::new();
    auditoria.comentario   = String::new();
    auditoria.resultado    = ResultadoAuditoria::Pendiente;
    auditoria.iniciado_en  = clock.unix_timestamp;
    auditoria.concluido_en = 0;
    auditoria.bump         = ctx.bumps.auditoria;

    let proveedor = &mut ctx.accounts.proveedor;
    if proveedor.estado == EstadoProveedor::Registrado {
        proveedor.estado = EstadoProveedor::EnRevision;
    }
    proveedor.actualizado_en = clock.unix_timestamp;

    ctx.accounts.platform.total_auditorias += 1;

    msg!(
        "Auditoría iniciada | Empresa: {} | Proveedor: {}",
        empresa_key,
        proveedor_key
    );
    Ok(())
}

pub fn concluir(
    ctx: Context<ConcluirAuditoria>,
    resultado: ResultadoAuditoria,
    hash_reporte: String,
    comentario: String,
) -> Result<()> {
    require!(hash_reporte.len() == 64, MarketplaceError::ReporteInvalido);
    require!(
        ctx.accounts.auditoria.estado != EstadoAuditoria::Concluida,
        MarketplaceError::AuditoriaYaConcluida
    );
    require!(
        ctx.accounts.empresa.autoridad == ctx.accounts.empresa_autoridad.key(),
        MarketplaceError::NoEsDuenoDeSolicitud
    );

    let clock = Clock::get()?;
    let proveedor_key = ctx.accounts.proveedor.key();

    let auditoria = &mut ctx.accounts.auditoria;
    auditoria.estado       = EstadoAuditoria::Concluida;
    auditoria.resultado    = resultado.clone();
    auditoria.hash_reporte = hash_reporte;
    auditoria.comentario   = comentario;
    auditoria.concluido_en = clock.unix_timestamp;

    let resultado_log = auditoria.resultado.clone();

    let proveedor = &mut ctx.accounts.proveedor;
    proveedor.total_auditorias += 1;
    proveedor.actualizado_en   = clock.unix_timestamp;

    match resultado {
        ResultadoAuditoria::Aprobado | ResultadoAuditoria::Observaciones => {
            proveedor.estado = EstadoProveedor::Verificado;
        }
        ResultadoAuditoria::Rechazado => {
            proveedor.estado = EstadoProveedor::Rechazado;
        }
        _ => {}
    }

    msg!(
        "Auditoría concluida | Resultado: {:?} | Proveedor: {}",
        resultado_log,
        proveedor_key
    );
    Ok(())
}

#[derive(Accounts)]
pub struct IniciarAuditoria<'info> {
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
        mut,
        seeds = [SEED_PROVEEDOR, &proveedor.id.to_le_bytes()],
        bump  = proveedor.bump,
    )]
    pub proveedor: Account<'info, Proveedor>,

    #[account(
        init,
        payer = empresa_autoridad,
        space = Auditoria::LEN,
        seeds = [SEED_AUDITORIA, empresa.key().as_ref(), proveedor.key().as_ref()],
        bump
    )]
    pub auditoria: Account<'info, Auditoria>,

    #[account(mut)]
    pub empresa_autoridad: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConcluirAuditoria<'info> {
    #[account(
        seeds = [SEED_EMPRESA, &empresa.id.to_le_bytes()],
        bump  = empresa.bump,
    )]
    pub empresa: Account<'info, Empresa>,

    #[account(
        mut,
        seeds = [SEED_PROVEEDOR, &proveedor.id.to_le_bytes()],
        bump  = proveedor.bump,
    )]
    pub proveedor: Account<'info, Proveedor>,

    #[account(
        mut,
        seeds = [SEED_AUDITORIA, empresa.key().as_ref(), proveedor.key().as_ref()],
        bump  = auditoria.bump,
    )]
    pub auditoria: Account<'info, Auditoria>,

    pub empresa_autoridad: Signer<'info>,
}