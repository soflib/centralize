# Marketplace Privado de Solicitaciones
### Solana · Anchor · Rust

Registro inmutable on-chain de membresías, auditorías, calificaciones
y solicitudes privadas de servicio entre empresas y proveedores.

---

## Estructura del proyecto

```
programs/marketplace_solicitaciones/src/
├── lib.rs                        ← Entry point: declara todas las instrucciones
├── errors.rs                     ← Errores personalizados del programa
├── state/
│   ├── mod.rs
│   ├── platform.rs               ← Config global + contadores
│   ├── empresa.rs                ← Cuenta de empresa suscriptora
│   ├── proveedor.rs              ← Cuenta de proveedor de servicios
│   ├── membresia.rs              ← Suscripción anual (empresa o proveedor)
│   ├── auditoria.rs              ← Proceso de verificación de proveedor
│   ├── calificacion.rs           ← Calificación post-servicio
│   └── solicitud.rs              ← Solicitud privada de servicio
└── instructions/
    ├── mod.rs
    ├── initialize.rs             ← initialize_platform
    ├── empresa.rs                ← register_empresa
    ├── proveedor.rs              ← register_proveedor
    ├── membresia.rs              ← activar + renovar membresía
    ├── auditoria.rs              ← iniciar + concluir auditoría
    ├── calificacion.rs           ← calificar_proveedor
    └── solicitud.rs              ← publicar + asignar + cerrar solicitud
```

---

## Flujo completo

```
[EMPRESA]                          [TU BACKEND]              [PROVEEDOR]
    │                                   │                        │
    ├─ register_empresa() ──────────────┤                        │
    │  (membresía = Pendiente)          │                        │
    │                                   │                        │
    │  Paga 300 MXN (MercadoPago) ─────►│                        │
    │                                   ├─ activar_membresia() ──┤
    │  (membresía = Activa ✓)           │                        │
    │                                   │                        ├─ register_proveedor()
    │                                   │                        │  (membresía = Pendiente)
    │                                   │                        │
    │                                   │  Paga 100 MXN ────────►│
    │                                   ├─ activar_membresia() ──┘
    │                                   │  (membresía = Activa ✓)
    │                                   │
    ├─ iniciar_auditoria() ─────────────┤
    │  (proveedor = EnRevision)         │
    │                                   │
    ├─ concluir_auditoria(Aprobado) ────┤
    │  (proveedor = Verificado ✓)       │
    │                                   │
    ├─ publicar_solicitud() ────────────┤
    │                                   │
    ├─ asignar_solicitud() ─────────────┤ (solo a proveedores Verificados)
    │                                   │
    ├─ cerrar_solicitud(completada) ────┤
    │                                   │
    └─ calificar_proveedor() ───────────┤
```

---

## Prerequisitos

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest
```

## Comandos

```bash
# Instalar dependencias
yarn install

# Compilar
anchor build

# Ejecutar tests
anchor test

# Desplegar en devnet
anchor deploy --provider.cluster devnet
```

---

## Modelo de pagos

Los pagos NO se procesan en Solana. El flujo es:

1. Usuario paga con **tarjeta o SPEI** (MercadoPago / Stripe / Conekta)
2. Tu backend recibe la confirmación del pago
3. Tu backend llama a `activar_membresia` con la referencia del pago
4. Solana registra de forma **inmutable** que esa membresía fue activada,
   cuándo, con qué comprobante y por cuánto tiempo

| Tipo        | Precio  | Duración |
|-------------|---------|----------|
| Empresa     | 300 MXN | 1 año    |
| Proveedor   | 100 MXN | 1 año    |

---

## Qué vive on-chain vs off-chain

| Dato                        | Dónde      |
|-----------------------------|------------|
| Estado de membresía         | On-chain   |
| Resultado de auditoría      | On-chain   |
| Calificaciones              | On-chain   |
| Estado de solicitudes       | On-chain   |
| Documentos (INE, actas)     | Off-chain (tu servidor / IPFS) |
| Hash SHA-256 de documentos  | On-chain (para verificar integridad) |
| Detalles completos solicitud| Off-chain  |

---

## PDAs (Program Derived Addresses)

```
PlatformConfig    → ["platform"]
Empresa           → ["empresa",      empresa_id_bytes]
Proveedor         → ["proveedor",    proveedor_id_bytes]
Membresia         → ["membresia",    autoridad_pubkey]
Auditoria         → ["auditoria",    empresa_pubkey, proveedor_pubkey]
Calificacion      → ["calificacion", empresa_pubkey, proveedor_pubkey]
SolicitudServicio → ["solicitud",    empresa_pubkey, solicitud_id_bytes]
```
