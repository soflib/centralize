use anchor_lang::prelude::*;
use crate::errors::MarketplaceError;
use crate::state::{
    empresa::{Empresa, SEED_EMPRESA},
    proveedor::{Proveedor, SEED_PROVEEDOR},
    membresia::{Membresia, SEED_MEMBRESIA},
    auditoria::{Auditoria, ResultadoAuditoria, SEED_AUDITORIA},
    calificacion::*,
};

pub fn calificar(
    ctx: Context<CalificarProveedor>,
    puntuacion: u8,
    puntuacion_cumplimiento: u8,
    puntuacion_calidad: u8,
    puntuacion_precio: u8,
    comentario: String,
    recomienda: bool,
) -> Result<()> {
    require!(
        ctx.accounts.auditoria.resultado == ResultadoAuditoria::Aprobado
        || ctx.accounts.auditoria.resultado == ResultadoAuditoria::Observaciones,
        MarketplaceError::ProveedorNoVerificado
    );

    for p in [puntuacion, puntuacion_cumplimiento, puntuacion_calidad, puntuacion_precio] {
        require!(p >= 1 && p <= 10, MarketplaceError::PuntuacionInvalida);
    }

    let clock = Clock::get()?;
    let empresa_key   = ctx.accounts.empresa.key();
    let proveedor_key = ctx.accounts.proveedor.key();

    let calificacion = &mut ctx.accounts.calificacion;
    calificacion.empresa                 = empresa_key;
    calificacion.proveedor               = proveedor_key;
    calificacion.puntuacion              = puntuacion;
    calificacion.puntuacion_cumplimiento = puntuacion_cumplimiento;
    calificacion.puntuacion_calidad      = puntuacion_calidad;
    calificacion.puntuacion_precio       = puntuacion_precio;
    calificacion.comentario              = comentario;
    calificacion.recomienda              = recomienda;
    calificacion.creado_en               = clock.unix_timestamp;
    calificacion.bump                    = ctx.bumps.calificacion;

    let proveedor = &mut ctx.accounts.proveedor;
    let n = proveedor.total_auditorias as u32;
    if n > 0 {
        let suma = proveedor.puntuacion_promedio as u32 * (n - 1) + puntuacion as u32 * 10;
        proveedor.puntuacion_promedio = (suma / n) as u8;
    } else {
        proveedor.puntuacion_promedio = puntuacion * 10;
    }

    msg!(
        "Calificación registrada | Proveedor: {} | Puntuación: {}/10 | Recomienda: {}",
        proveedor_key,
        puntuacion,
        recomienda
    );
    Ok(())
}

#[derive(Accounts)]
pub struct CalificarProveedor<'info> {
    #[account(
        seeds = [SEED_EMPRESA, &empresa.id.to_le_bytes()],
        bump  = empresa.bump,
        constraint = empresa.autoridad == empresa_autoridad.key()
    )]
    pub empresa: Account<'info, Empresa>,

    #[account(
        mut,
        seeds = [SEED_PROVEEDOR, &proveedor.id.to_le_bytes()],
        bump  = proveedor.bump,
    )]
    pub proveedor: Account<'info, Proveedor>,

    #[account(
        seeds = [SEED_AUDITORIA, empresa.key().as_ref(), proveedor.key().as_ref()],
        bump  = auditoria.bump,
    )]
    pub auditoria: Account<'info, Auditoria>,

    #[account(
        init,
        payer = empresa_autoridad,
        space = Calificacion::LEN,
        seeds = [SEED_CALIFICACION, empresa.key().as_ref(), proveedor.key().as_ref()],
        bump
    )]
    pub calificacion: Account<'info, Calificacion>,

    #[account(mut)]
    pub empresa_autoridad: Signer<'info>,

    pub system_program: Program<'info, System>,
}