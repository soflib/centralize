// ============================================================
// MARKETPLACE PRIVADO DE SOLICITACIONES
// Programa Anchor · Solana · Rust
//
// Registro inmutable de membresías, auditorías,
// calificaciones y solicitudes privadas de servicio.
// ============================================================


use anchor_lang::prelude::*;

pub mod errors;
pub mod state;
pub mod instructions;

use instructions::*;
use state::auditoria::ResultadoAuditoria;

declare_id!("AFEkWVEezVDdXCvEZajqoQUkLDuAXLEkD1Gry6B3MqcD");

#[program]
pub mod marketplace_solicitaciones {
    use super::*;

    // ── Plataforma ───────────────────────────────────────────
    /// Inicializa la plataforma. Se llama UNA sola vez al desplegar.
    pub fn initialize_platform(ctx: Context<InitializePlatform>) -> Result<()> {
        initialize::handler(ctx)
    }

    // ── Empresas ─────────────────────────────────────────────
    /// Registra una nueva empresa (membresía queda pendiente hasta confirmar pago).
    pub fn register_empresa(
        ctx: Context<RegisterEmpresa>,
        nombre: String,
        rfc: String,
        categoria: String,
        hash_doc_constitucion: String,
    ) -> Result<()> {
        empresa::register(ctx, nombre, rfc, categoria, hash_doc_constitucion)
    }

    // ── Proveedores ──────────────────────────────────────────
    /// Registra un nuevo proveedor (membresía queda pendiente hasta confirmar pago).
    pub fn register_proveedor(
        ctx: Context<RegisterProveedor>,
        nombre: String,
        rfc: String,
        descripcion: String,
        categoria: String,
        hash_doc_identidad: String,
        hash_doc_experiencia: String,
    ) -> Result<()> {
        proveedor::register(ctx, nombre, rfc, descripcion, categoria, hash_doc_identidad, hash_doc_experiencia)
    }

    // ── Membresías ───────────────────────────────────────────
    /// Admin activa la membresía de una empresa tras confirmar pago fiat.
    pub fn activar_membresia_empresa(
        ctx: Context<ActivarEmpresa>,
        hash_comprobante: String,
        referencia_pago: String,
    ) -> Result<()> {
        membresia::activar_empresa(ctx, hash_comprobante, referencia_pago)
    }

    /// Admin activa la membresía de un proveedor tras confirmar pago fiat.
    pub fn activar_membresia_proveedor(
        ctx: Context<ActivarProveedor>,
        hash_comprobante: String,
        referencia_pago: String,
    ) -> Result<()> {
        membresia::activar_proveedor(ctx, hash_comprobante, referencia_pago)
    }

    /// Admin renueva cualquier membresía (empresa o proveedor).
    pub fn renovar_membresia(
        ctx: Context<RenovarMembresia>,
        hash_comprobante: String,
        referencia_pago: String,
    ) -> Result<()> {
        membresia::renovar(ctx, hash_comprobante, referencia_pago)
    }

    // ── Auditorías ───────────────────────────────────────────
    /// Empresa con membresía activa inicia la auditoría de un proveedor.
    pub fn iniciar_auditoria(ctx: Context<IniciarAuditoria>) -> Result<()> {
        auditoria::iniciar(ctx)
    }

    /// Empresa registra el resultado de la auditoría con hash del reporte.
    pub fn concluir_auditoria(
        ctx: Context<ConcluirAuditoria>,
        resultado: ResultadoAuditoria,
        hash_reporte: String,
        comentario: String,
    ) -> Result<()> {
        auditoria::concluir(ctx, resultado, hash_reporte, comentario)
    }

    // ── Calificaciones ────────────────────────────────────────
    /// Empresa califica a un proveedor después de recibir el servicio.
    pub fn calificar_proveedor(
        ctx: Context<CalificarProveedor>,
        puntuacion: u8,
        puntuacion_cumplimiento: u8,
        puntuacion_calidad: u8,
        puntuacion_precio: u8,
        comentario: String,
        recomienda: bool,
    ) -> Result<()> {
        calificacion::calificar(
            ctx,
            puntuacion,
            puntuacion_cumplimiento,
            puntuacion_calidad,
            puntuacion_precio,
            comentario,
            recomienda,
        )
    }

    // ── Solicitudes ───────────────────────────────────────────
    /// Empresa publica una solicitud privada de servicio.
    pub fn publicar_solicitud(
        ctx: Context<PublicarSolicitud>,
        categoria: String,
        hash_detalle: String,
        descripcion_breve: String,
        fecha_limite: i64,
    ) -> Result<()> {
        solicitud::publicar(ctx, categoria, hash_detalle, descripcion_breve, fecha_limite)
    }

    /// Empresa asigna la solicitud a un proveedor verificado.
    pub fn asignar_solicitud(ctx: Context<AsignarSolicitud>) -> Result<()> {
        solicitud::asignar(ctx)
    }

    /// Empresa cierra la solicitud como completada o cancelada.
    pub fn cerrar_solicitud(
        ctx: Context<CerrarSolicitud>,
        completada: bool,
    ) -> Result<()> {
        solicitud::cerrar(ctx, completada)
    }
}
