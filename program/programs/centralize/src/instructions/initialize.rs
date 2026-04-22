use anchor_lang::prelude::*;
use crate::state::platform::*;

// ────────────────────────────────────────────────────────────
// initialize_platform
// Se llama UNA sola vez al desplegar el programa.
// Crea la cuenta PlatformConfig con el admin = quien firma.
// ────────────────────────────────────────────────────────────

pub fn handler(ctx: Context<InitializePlatform>) -> Result<()> {
    let platform = &mut ctx.accounts.platform;
    let clock = Clock::get()?;

    platform.admin        = ctx.accounts.admin.key();
    platform.activo       = true;
    platform.version      = 1;
    platform.total_empresas    = 0;
    platform.total_proveedores = 0;
    platform.total_auditorias  = 0;
    platform.creado_en    = clock.unix_timestamp;
    platform.bump         = ctx.bumps.platform;

    msg!("Plataforma inicializada. Admin: {}", platform.admin);
    Ok(())
}

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(
        init,
        payer  = admin,
        space  = PlatformConfig::LEN,
        seeds  = [SEED_PLATFORM],
        bump
    )]
    pub platform: Account<'info, PlatformConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}
