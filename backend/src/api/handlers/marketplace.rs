// ============================================================
// api/handlers/marketplace.rs
// ============================================================

use std::sync::Arc;
use axum::{http::StatusCode, extract::State, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::models::{AppState, ErrorResponse};

#[derive(Deserialize)]
pub struct RegisterEmpresaRequest {
    pub nombre:                String,
    pub rfc:                   String,
    pub categoria:             String,
    pub hash_doc_constitucion: String,
}

#[derive(Deserialize)]
pub struct RegisterProveedorRequest {
    pub nombre:               String,
    pub rfc:                  String,
    pub descripcion:          String,
    pub categoria:            String,
    pub hash_doc_identidad:   String,
    pub hash_doc_experiencia: String,
}

#[derive(Deserialize)]
pub struct ActivarMembresiaRequest {
    pub wallet:           String,
    pub hash_comprobante: String,
    pub referencia_pago:  String,
}

#[derive(Deserialize)]
pub struct RenovarMembresiaRequest {
    pub wallet:           String,
    pub hash_comprobante: String,
    pub referencia_pago:  String,
}

#[derive(Deserialize)]
pub struct IniciarAuditoriaRequest {
    pub empresa_id:   u64,
    pub proveedor_id: u64,
}

#[derive(Deserialize)]
pub struct ConcluirAuditoriaRequest {
    pub empresa_id:   u64,
    pub proveedor_id: u64,
    pub resultado:    String,
    pub hash_reporte: String,
    pub comentario:   String,
}

#[derive(Deserialize)]
pub struct CalificarProveedorRequest {
    pub empresa_id:              u64,
    pub proveedor_id:            u64,
    pub puntuacion:              u8,
    pub puntuacion_cumplimiento: u8,
    pub puntuacion_calidad:      u8,
    pub puntuacion_precio:       u8,
    pub comentario:              String,
    pub recomienda:              bool,
}

#[derive(Deserialize)]
pub struct PublicarSolicitudRequest {
    pub empresa_id:        u64,
    pub categoria:         String,
    pub hash_detalle:      String,
    pub descripcion_breve: String,
    pub fecha_limite:      i64,
}

#[derive(Deserialize)]
pub struct AsignarSolicitudRequest {
    pub empresa_id:   u64,
    pub proveedor_id: u64,
    pub solicitud_id: String,
}

#[derive(Deserialize)]
pub struct CerrarSolicitudRequest {
    pub empresa_id:   u64,
    pub solicitud_id: String,
    pub completada:   bool,
}

#[derive(Serialize)]
pub struct TxResponse {
    pub success:      bool,
    pub tx_signature: String,
    pub timestamp:    String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mensaje:      Option<String>,
}

fn tx_ok(sig: String, mensaje: impl Into<String>) -> (StatusCode, Json<TxResponse>) {
    (StatusCode::OK, Json(TxResponse {
        success:      true,
        tx_signature: sig,
        timestamp:    Utc::now().to_rfc3339(),
        mensaje:      Some(mensaje.into()),
    }))
}

fn bad_req(msg: &str) -> (StatusCode, Json<ErrorResponse>) {
    (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: msg.into() }))
}

pub async fn initialize_platform(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    let sig = state.solana.initialize_platform();
    Ok(tx_ok(sig, "Plataforma inicializada"))
}

pub async fn register_empresa(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterEmpresaRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.nombre.is_empty() || body.rfc.is_empty() {
        return Err(bad_req("nombre y rfc son requeridos"));
    }
    let sig = state.solana.register_empresa(body.nombre, body.rfc, body.categoria, body.hash_doc_constitucion);
    Ok(tx_ok(sig, "Empresa registrada — membresía pendiente de activación"))
}

pub async fn register_proveedor(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterProveedorRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.nombre.is_empty() || body.rfc.is_empty() {
        return Err(bad_req("nombre y rfc son requeridos"));
    }
    let sig = state.solana.register_proveedor(body.nombre, body.rfc, body.descripcion, body.categoria, body.hash_doc_identidad, body.hash_doc_experiencia);
    Ok(tx_ok(sig, "Proveedor registrado — membresía pendiente de activación"))
}

pub async fn activar_membresia_empresa(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ActivarMembresiaRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.wallet.is_empty() || body.hash_comprobante.is_empty() {
        return Err(bad_req("wallet y hash_comprobante son requeridos"));
    }
    let sig = state.solana.activar_membresia_empresa(&body.wallet, body.hash_comprobante, body.referencia_pago);
    Ok(tx_ok(sig, "Membresía de empresa activada"))
}

pub async fn activar_membresia_proveedor(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ActivarMembresiaRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.wallet.is_empty() || body.hash_comprobante.is_empty() {
        return Err(bad_req("wallet y hash_comprobante son requeridos"));
    }
    let sig = state.solana.activar_membresia_proveedor(&body.wallet, body.hash_comprobante, body.referencia_pago);
    Ok(tx_ok(sig, "Membresía de proveedor activada"))
}

pub async fn renovar_membresia(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RenovarMembresiaRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.wallet.is_empty() {
        return Err(bad_req("wallet es requerido"));
    }
    let sig = state.solana.renovar_membresia(&body.wallet, body.hash_comprobante, body.referencia_pago);
    Ok(tx_ok(sig, "Membresía renovada"))
}

pub async fn iniciar_auditoria(
    State(state): State<Arc<AppState>>,
    Json(body): Json<IniciarAuditoriaRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    let sig = state.solana.iniciar_auditoria(body.empresa_id, body.proveedor_id);
    Ok(tx_ok(sig, "Auditoría iniciada"))
}

pub async fn concluir_auditoria(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ConcluirAuditoriaRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.hash_reporte.is_empty() || body.resultado.is_empty() {
        return Err(bad_req("hash_reporte y resultado son requeridos"));
    }
    if !["Aprobado", "Rechazado", "Observaciones"].contains(&body.resultado.as_str()) {
        return Err(bad_req("resultado debe ser: Aprobado | Rechazado | Observaciones"));
    }
    let sig = state.solana.concluir_auditoria(body.empresa_id, body.proveedor_id, body.resultado, body.hash_reporte, body.comentario);
    Ok(tx_ok(sig, "Auditoría concluida"))
}

pub async fn calificar_proveedor(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CalificarProveedorRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    for p in [body.puntuacion, body.puntuacion_cumplimiento, body.puntuacion_calidad, body.puntuacion_precio] {
        if !(1..=5).contains(&p) {
            return Err(bad_req("Las puntuaciones deben estar entre 1 y 5"));
        }
    }
    let sig = state.solana.calificar_proveedor(body.empresa_id, body.proveedor_id, body.puntuacion, body.puntuacion_cumplimiento, body.puntuacion_calidad, body.puntuacion_precio, body.comentario, body.recomienda);
    Ok(tx_ok(sig, "Calificación registrada on-chain"))
}

pub async fn publicar_solicitud(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PublicarSolicitudRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.hash_detalle.is_empty() || body.categoria.is_empty() {
        return Err(bad_req("hash_detalle y categoria son requeridos"));
    }
    if body.fecha_limite <= Utc::now().timestamp() {
        return Err(bad_req("fecha_limite debe ser una fecha futura"));
    }
    let sig = state.solana.publicar_solicitud(body.empresa_id, body.categoria, body.hash_detalle, body.descripcion_breve, body.fecha_limite);
    Ok(tx_ok(sig, "Solicitud publicada on-chain"))
}

pub async fn asignar_solicitud(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AsignarSolicitudRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.solicitud_id.is_empty() {
        return Err(bad_req("solicitud_id es requerido"));
    }
    let sig = state.solana.asignar_solicitud(body.empresa_id, body.proveedor_id, body.solicitud_id);
    Ok(tx_ok(sig, "Solicitud asignada al proveedor"))
}

pub async fn cerrar_solicitud(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CerrarSolicitudRequest>,
) -> Result<(StatusCode, Json<TxResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.solicitud_id.is_empty() {
        return Err(bad_req("solicitud_id es requerido"));
    }
    let sig = state.solana.cerrar_solicitud(body.empresa_id, body.solicitud_id, body.completada);
    let estado = if body.completada { "completada" } else { "cancelada" };
    Ok(tx_ok(sig, format!("Solicitud {}", estado)))
}