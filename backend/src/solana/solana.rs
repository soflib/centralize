// ============================================================
//  solana.rs
// ============================================================

use std::fs;
use anyhow::{Result, anyhow};
use borsh::BorshSerialize;
use sha2::{Sha256, Digest};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;
use tracing::{info, warn};

pub struct SolanaClient {
    pub rpc:        RpcClient,
    pub payer:      Keypair,
    pub program_id: Pubkey,
}

fn discriminator(name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", name));
    let result = hasher.finalize();
    result[..8].try_into().unwrap()
}

fn send_ix(rpc: &RpcClient, payer: &Keypair, ix: Instruction) -> Result<String> {
    let blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );
    let sig = rpc.send_and_confirm_transaction(&tx)?;
    Ok(sig.to_string())
}

impl SolanaClient {
    pub fn new(rpc_url: &str, program_id_str: &str) -> Result<Self> {
        let rpc = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );
        let keypair_path = std::env::var("KEYPAIR_PATH")
            .unwrap_or_else(|_| "/app/secrets/id.json".to_string());
        let keypair_bytes = fs::read_to_string(&keypair_path)
            .map_err(|e| anyhow!("No se pudo leer keypair en {}: {}", keypair_path, e))?;
        let bytes: Vec<u8> = serde_json::from_str(&keypair_bytes)
            .map_err(|e| anyhow!("Keypair JSON inválido: {}", e))?;
        let payer = Keypair::from_bytes(&bytes)
            .map_err(|e| anyhow!("Keypair bytes inválidos: {}", e))?;
        let program_id = Pubkey::from_str(program_id_str)
            .map_err(|e| anyhow!("Program ID inválido: {}", e))?;
        info!("Solana client inicializado — payer: {}", payer.pubkey());
        Ok(Self { rpc, payer, program_id })
    }

    // ── PDAs ─────────────────────────────────────────────────

    fn platform_pda(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"platform"], &self.program_id).0
    }

    // empresa usa ID secuencial, no la wallet
    fn empresa_pda(&self, id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[b"empresa", &id.to_le_bytes()],
            &self.program_id,
        ).0
    }

    // proveedor usa ID secuencial también — muéstrame proveedor.rs si falla
    fn proveedor_pda(&self, id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[b"proveedor", &id.to_le_bytes()],
            &self.program_id,
        ).0
    }

    fn membresia_pda(&self, autoridad: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[b"membresia", autoridad.as_ref()],
            &self.program_id,
        ).0
    }

    fn auditoria_pda(&self, empresa: &Pubkey, proveedor: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[b"auditoria", empresa.as_ref(), proveedor.as_ref()],
            &self.program_id,
        ).0
    }

    fn calificacion_pda(&self, empresa: &Pubkey, proveedor: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[b"calificacion", empresa.as_ref(), proveedor.as_ref()],
            &self.program_id,
        ).0
    }

    fn solicitud_pda(&self, empresa: &Pubkey, id: &str) -> Pubkey {
        Pubkey::find_program_address(
            &[b"solicitud", empresa.as_ref(), id.as_bytes()],
            &self.program_id,
        ).0
    }

    // ── Lee total_empresas del platform account ───────────────
    // Layout después del discriminador (8 bytes):
    // admin(32) + activo(1) + version(1) + total_empresas(8) → offset 42
    fn read_total_empresas(&self) -> u64 {
        let pda = self.platform_pda();
        match self.rpc.get_account_data(&pda) {
            Ok(data) => {
                if data.len() >= 50 {
                    u64::from_le_bytes(data[42..50].try_into().unwrap_or([0; 8]))
                } else {
                    warn!("Platform account demasiado pequeño");
                    0
                }
            }
            Err(e) => {
                warn!("No se pudo leer platform: {}", e);
                0
            }
        }
    }

    // ── Lee total_proveedores del platform account ────────────
    // total_proveedores está en offset 50
    fn read_total_proveedores(&self) -> u64 {
        let pda = self.platform_pda();
        match self.rpc.get_account_data(&pda) {
            Ok(data) => {
                if data.len() >= 58 {
                    u64::from_le_bytes(data[50..58].try_into().unwrap_or([0; 8]))
                } else {
                    0
                }
            }
            Err(e) => {
                warn!("No se pudo leer platform: {}", e);
                0
            }
        }
    }

    // ── Helper send o sim ─────────────────────────────────────
    fn send_or_sim(&self, ix: Instruction) -> String {
        match send_ix(&self.rpc, &self.payer, ix) {
            Ok(sig) => { info!("✅ Solana tx: {}", sig); sig }
            Err(e)  => {
                warn!("⚠️  Solana falló, simulando: {}", e);
                let random_bytes: [u8; 16] = rand::random();
                format!("SIM-{}", hex::encode(random_bytes))
            }
        }
    }

    pub fn initialize_platform(&self) -> String {
        let data = discriminator("initialize_platform").to_vec();
        let platform_pda = self.platform_pda();
        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(platform_pda,                false),
                AccountMeta::new(self.payer.pubkey(),         true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    // ─────────────────────────────────────────────────────────
    // register_empresa
    // ─────────────────────────────────────────────────────────
    pub fn register_empresa(
        &self,
        nombre: String,
        rfc: String,
        categoria: String,
        hash_doc_constitucion: String,
    ) -> String {
        let total = self.read_total_empresas();

        let mut data = discriminator("register_empresa").to_vec();
        nombre.serialize(&mut data).unwrap();
        rfc.serialize(&mut data).unwrap();
        categoria.serialize(&mut data).unwrap();
        hash_doc_constitucion.serialize(&mut data).unwrap();

        let platform_pda  = self.platform_pda();
        let empresa_pda   = self.empresa_pda(total);
        let membresia_pda = self.membresia_pda(&self.payer.pubkey());

        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(platform_pda,                false),
                AccountMeta::new(empresa_pda,                 false),
                AccountMeta::new(membresia_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    // ─────────────────────────────────────────────────────────
    // register_proveedor
    // ─────────────────────────────────────────────────────────
    pub fn register_proveedor(
        &self,
        nombre: String,
        rfc: String,
        descripcion: String,
        categoria: String,
        hash_doc_identidad: String,
        hash_doc_experiencia: String,
    ) -> String {
        let total = self.read_total_proveedores();

        let mut data = discriminator("register_proveedor").to_vec();
        nombre.serialize(&mut data).unwrap();
        rfc.serialize(&mut data).unwrap();
        descripcion.serialize(&mut data).unwrap();
        categoria.serialize(&mut data).unwrap();
        hash_doc_identidad.serialize(&mut data).unwrap();
        hash_doc_experiencia.serialize(&mut data).unwrap();

        let platform_pda  = self.platform_pda();
        let proveedor_pda = self.proveedor_pda(total);
        let membresia_pda = self.membresia_pda(&self.payer.pubkey());

        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(platform_pda,                false),
                AccountMeta::new(proveedor_pda,               false),
                AccountMeta::new(membresia_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    // ─────────────────────────────────────────────────────────
    // activar_membresia_empresa
    // ─────────────────────────────────────────────────────────
    pub fn activar_membresia_empresa(
        &self,
        wallet: &str,
        hash_comprobante: String,
        referencia_pago: String,
    ) -> String {
        let wallet_pk = match Pubkey::from_str(wallet) {
            Ok(p) => p,
            Err(_) => return "ERR-wallet-invalida".to_string(),
        };
        let mut data = discriminator("activar_membresia_empresa").to_vec();
        hash_comprobante.serialize(&mut data).unwrap();
        referencia_pago.serialize(&mut data).unwrap();

        let membresia_pda = self.membresia_pda(&wallet_pk);

        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(membresia_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    // ─────────────────────────────────────────────────────────
    // activar_membresia_proveedor
    // ─────────────────────────────────────────────────────────
    pub fn activar_membresia_proveedor(
        &self,
        wallet: &str,
        hash_comprobante: String,
        referencia_pago: String,
    ) -> String {
        let wallet_pk = match Pubkey::from_str(wallet) {
            Ok(p) => p,
            Err(_) => return "ERR-wallet-invalida".to_string(),
        };
        let mut data = discriminator("activar_membresia_proveedor").to_vec();
        hash_comprobante.serialize(&mut data).unwrap();
        referencia_pago.serialize(&mut data).unwrap();

        let membresia_pda = self.membresia_pda(&wallet_pk);

        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(membresia_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    // ─────────────────────────────────────────────────────────
    // renovar_membresia
    // ─────────────────────────────────────────────────────────
    pub fn renovar_membresia(
        &self,
        wallet: &str,
        hash_comprobante: String,
        referencia_pago: String,
    ) -> String {
        let wallet_pk = match Pubkey::from_str(wallet) {
            Ok(p) => p,
            Err(_) => return "ERR-wallet-invalida".to_string(),
        };
        let mut data = discriminator("renovar_membresia").to_vec();
        hash_comprobante.serialize(&mut data).unwrap();
        referencia_pago.serialize(&mut data).unwrap();

        let membresia_pda = self.membresia_pda(&wallet_pk);

        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(membresia_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    // ─────────────────────────────────────────────────────────
    // iniciar_auditoria
    // ─────────────────────────────────────────────────────────
    pub fn iniciar_auditoria(&self, empresa_id: u64, proveedor_id: u64) -> String {
        let data = discriminator("iniciar_auditoria").to_vec();
        let platform_pda  = self.platform_pda();
        let empresa_pda   = self.empresa_pda(empresa_id);
        let proveedor_pda = self.proveedor_pda(proveedor_id);
        let membresia_pda = self.membresia_pda(&self.payer.pubkey());
        let auditoria_pda = self.auditoria_pda(&empresa_pda, &proveedor_pda);
        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(platform_pda,                false),
                AccountMeta::new_readonly(empresa_pda,        false),
                AccountMeta::new_readonly(membresia_pda,      false),
                AccountMeta::new(proveedor_pda,               false),
                AccountMeta::new(auditoria_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    pub fn concluir_auditoria(&self, empresa_id: u64, proveedor_id: u64, resultado: String, hash_reporte: String, comentario: String) -> String {
        let resultado_u8: u8 = match resultado.as_str() {
            "Aprobado"      => 1,
            "Rechazado"     => 2,
            "Observaciones" => 3,
            _               => 0,
        };
        let mut data = discriminator("concluir_auditoria").to_vec();
        resultado_u8.serialize(&mut data).unwrap();
        hash_reporte.serialize(&mut data).unwrap();
        comentario.serialize(&mut data).unwrap();
        let empresa_pda   = self.empresa_pda(empresa_id);
        let proveedor_pda = self.proveedor_pda(proveedor_id);
        let auditoria_pda = self.auditoria_pda(&empresa_pda, &proveedor_pda);
        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new_readonly(empresa_pda,        false),
                AccountMeta::new(proveedor_pda,               false),
                AccountMeta::new(auditoria_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    pub fn calificar_proveedor(&self, empresa_id: u64, proveedor_id: u64, puntuacion: u8, puntuacion_cumplimiento: u8, puntuacion_calidad: u8, puntuacion_precio: u8, comentario: String, recomienda: bool) -> String {
        let mut data = discriminator("calificar_proveedor").to_vec();
        puntuacion.serialize(&mut data).unwrap();
        puntuacion_cumplimiento.serialize(&mut data).unwrap();
        puntuacion_calidad.serialize(&mut data).unwrap();
        puntuacion_precio.serialize(&mut data).unwrap();
        comentario.serialize(&mut data).unwrap();
        recomienda.serialize(&mut data).unwrap();
        let empresa_pda      = self.empresa_pda(empresa_id);
        let proveedor_pda    = self.proveedor_pda(proveedor_id);
        let auditoria_pda    = self.auditoria_pda(&empresa_pda, &proveedor_pda);
        let calificacion_pda = self.calificacion_pda(&empresa_pda, &proveedor_pda);
        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new_readonly(empresa_pda,        false),
                AccountMeta::new(proveedor_pda,               false),
                AccountMeta::new_readonly(auditoria_pda,      false),
                AccountMeta::new(calificacion_pda,            false),
                AccountMeta::new(self.payer.pubkey(),         true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    pub fn publicar_solicitud(&self, empresa_id: u64, categoria: String, hash_detalle: String, descripcion_breve: String, fecha_limite: i64) -> String {
        let mut data = discriminator("publicar_solicitud").to_vec();
        categoria.serialize(&mut data).unwrap();
        hash_detalle.serialize(&mut data).unwrap();
        descripcion_breve.serialize(&mut data).unwrap();
        fecha_limite.serialize(&mut data).unwrap();
        let platform_pda  = self.platform_pda();
        let empresa_pda   = self.empresa_pda(empresa_id);
        let membresia_pda = self.membresia_pda(&self.payer.pubkey());
        let solicitud_pda = self.solicitud_pda(&empresa_pda, &hash_detalle);
        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(platform_pda,                false),
                AccountMeta::new_readonly(empresa_pda,        false),
                AccountMeta::new_readonly(membresia_pda,      false),
                AccountMeta::new(solicitud_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    pub fn asignar_solicitud(&self, empresa_id: u64, proveedor_id: u64, solicitud_id: String) -> String {
        let data = discriminator("asignar_solicitud").to_vec();
        let empresa_pda   = self.empresa_pda(empresa_id);
        let proveedor_pda = self.proveedor_pda(proveedor_id);
        let solicitud_pda = self.solicitud_pda(&empresa_pda, &solicitud_id);
        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new_readonly(empresa_pda,        false),
                AccountMeta::new_readonly(proveedor_pda,      false),
                AccountMeta::new(solicitud_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
            ],
            data,
        };
        self.send_or_sim(ix)
    }

    pub fn cerrar_solicitud(&self, empresa_id: u64, solicitud_id: String, completada: bool) -> String {
        let mut data = discriminator("cerrar_solicitud").to_vec();
        completada.serialize(&mut data).unwrap();
        let empresa_pda   = self.empresa_pda(empresa_id);
        let solicitud_pda = self.solicitud_pda(&empresa_pda, &solicitud_id);
        let ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new_readonly(empresa_pda,        false),
                AccountMeta::new(solicitud_pda,               false),
                AccountMeta::new(self.payer.pubkey(),         true),
            ],
            data,
        };
        self.send_or_sim(ix)
    }
}