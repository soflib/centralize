// ============================================================
// router.rs
//
// Router Axum unificado.
// Agrupa las rutas de tickets (existentes) y las nuevas rutas
// del marketplace de solicitaciones, siguiendo el mismo patrón.
// ============================================================

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::api::handlers::marketplace;
use crate::models::AppState;

pub fn build_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/marketplace/initialize", post(marketplace::initialize_platform))
        // ── Marketplace · Empresas ────────────────────────────
        .route(
            "/api/marketplace/empresa/register",
            post(marketplace::register_empresa),
        )

        // ── Marketplace · Proveedores ─────────────────────────
        .route(
            "/api/marketplace/proveedor/register",
            post(marketplace::register_proveedor),
        )

        // ── Marketplace · Membresías ──────────────────────────
        // Admin activa empresa tras confirmar pago fiat
        .route(
            "/api/marketplace/membresia/activar/empresa",
            post(marketplace::activar_membresia_empresa),
        )
        // Admin activa proveedor tras confirmar pago fiat
        .route(
            "/api/marketplace/membresia/activar/proveedor",
            post(marketplace::activar_membresia_proveedor),
        )
        // Admin renueva cualquier membresía
        .route(
            "/api/marketplace/membresia/renovar",
            post(marketplace::renovar_membresia),
        )

        // ── Marketplace · Auditorías ──────────────────────────
        .route(
            "/api/marketplace/auditoria/iniciar",
            post(marketplace::iniciar_auditoria),
        )
        .route(
            "/api/marketplace/auditoria/concluir",
            post(marketplace::concluir_auditoria),
        )

        // ── Marketplace · Calificaciones ──────────────────────
        .route(
            "/api/marketplace/calificacion",
            post(marketplace::calificar_proveedor),
        )

        // ── Marketplace · Solicitudes ─────────────────────────
        .route(
            "/api/marketplace/solicitud/publicar",
            post(marketplace::publicar_solicitud),
        )
        .route(
            "/api/marketplace/solicitud/asignar",
            post(marketplace::asignar_solicitud),
        )
        .route(
            "/api/marketplace/solicitud/cerrar",
            post(marketplace::cerrar_solicitud),
        )

        // ── Middleware ────────────────────────────────────────
        .layer(cors)
        .with_state(state)
}