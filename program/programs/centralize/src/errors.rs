use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    // ── Plataforma ──────────────────────────────────────────
    #[msg("La plataforma está desactivada")]
    PlataformaInactiva,

    #[msg("Solo el administrador puede ejecutar esta acción")]
    NoEsAdmin,

    // ── Membresía ───────────────────────────────────────────
    #[msg("La membresía no está activa")]
    MembresiaInactiva,

    #[msg("La membresía ya está activa")]
    MembresiaYaActiva,

    #[msg("La membresía ha vencido")]
    MembresiaVencida,

    #[msg("Referencia de pago inválida o vacía")]
    ReferenciaPagoInvalida,

    // ── Empresa ─────────────────────────────────────────────
    #[msg("La empresa no está activa")]
    EmpresaNoActiva,

    #[msg("RFC inválido (máximo 13 caracteres)")]
    RfcInvalido,

    #[msg("Nombre inválido o vacío")]
    NombreInvalido,

    // ── Proveedor ───────────────────────────────────────────
    #[msg("El proveedor no está verificado")]
    ProveedorNoVerificado,

    #[msg("El proveedor está suspendido")]
    ProveedorSuspendido,

    #[msg("El proveedor ya tiene una auditoría activa con esta empresa")]
    AuditoriaYaExiste,

    // ── Auditoría ───────────────────────────────────────────
    #[msg("La auditoría ya fue concluida")]
    AuditoriaYaConcluida,

    #[msg("La auditoría no está en proceso")]
    AuditoriaNoEnProceso,

    #[msg("Debe proporcionar un hash de reporte válido")]
    ReporteInvalido,

    // ── Solicitud ───────────────────────────────────────────
    #[msg("La solicitud no está abierta")]
    SolicitudNoAbierta,

    #[msg("La solicitud ya fue cerrada")]
    SolicitudCerrada,

    #[msg("La fecha límite debe ser futura")]
    FechaLimiteInvalida,

    #[msg("Solo la empresa dueña puede modificar esta solicitud")]
    NoEsDuenoDeSolicitud,

    // ── Calificación ─────────────────────────────────────────
    #[msg("Puntuación inválida (debe ser entre 1 y 10)")]
    PuntuacionInvalida,

    #[msg("Ya existe una calificación de esta empresa para este proveedor")]
    CalificacionYaExiste,

    // ── General ──────────────────────────────────────────────
    #[msg("Descripción demasiado larga")]
    DescripcionMuyLarga,

    #[msg("Hash de documento inválido")]
    HashDocumentoInvalido,
}
