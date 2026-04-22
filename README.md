# Marketplace Privado de Solicitaciones
### Solana · Anchor · Rust

EL objetivo del proyecto y el cual es mas grande que solo algo sencillo, es:
1. buscar / explorar un proyecto mas complejo y el comprotamiento de Solana
2. poder realziar el proyecto asi como asesoria y redes privadas (esa fue la razon de levantar solana en docker aunque no es para redes privadas en PROD si bien poder comenzar en DEV).
3. poder conseguir algun empleo en solana Rust y poder tener un proyecto mas complejo de estudio.

Registro inmutable on-chain de membresías, auditorías, calificaciones
y solicitudes privadas de servicio entre empresas y proveedores.

REFERCIAN E INTRO A LA PRIMER ETAPA DEL PROYECTO
centralize-intro.html (Open with live server)

Aun en desarrollo y mucha logica y arquitectura por mejorar, de tener problemas favor de comunicarse con:

Gerardo Ruiz Bustani
email: gbustsni@soflib.com (email de proyectos)
email: solbustsni@gmail.com (email personal)
what'sApp: +52 55 1689 8561
www.soflib.com

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
# Centralize — Marketplace Privado de Solicitaciones

> Plataforma descentralizada para registrar empresas y proveedores, publicar licitaciones privadas, auditar servicios y calificar proveedores — con registro inmutable en **Solana**.

---

## ¿Qué es esto?

**Centralize** es un marketplace B2B construido sobre blockchain. Cada acción — registro, membresía, auditoría, calificación — queda registrada permanentemente en Solana. No hay base de datos central que pueda ser alterada.

### Stack técnico

| Capa | Tecnología |
|------|-----------|
| Blockchain | Solana (localnet) |
| Smart contract | Anchor + Rust |
| API REST | Axum (Rust) |
| Base de datos | PostgreSQL |
| Infraestructura | Docker Compose |

---

## Configuración

Copia el archivo de ejemplo y llena los valores:

```bash
cp infrastructure/dev/.env.example infrastructure/dev/.env
```

El `.env.example` contiene todas las variables necesarias con valores de ejemplo.

---

## Setup — Primera vez

Sigue estos pasos **en orden**.

### 1. Instalar Solana

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
```

### 2. Crear keypair de wallet

```bash
# Si ya tienes keypair, solo corre: solana address
solana-keygen new --outfile ~/.config/solana/id.json --no-bip39-passphrase

solana config set --url http://localhost:8899
```

### 3. Compilar el contrato

```bash
cd program/
anchor build
```

### 4. Generar Program ID fijo

Este ID es **permanente** — identifica tu programa en la blockchain.

```bash
# Generar keypair del programa
solana-keygen new \
  --outfile target/deploy/centralize-keypair.json \
  --no-bip39-passphrase --force

# Ver el Program ID
solana-keygen pubkey target/deploy/centralize-keypair.json
# Ejemplo: AFEkWVEezVDdXCvEZajqoQUkLDuAXLEkD1Gry6B3MqcD
```

### 5. Actualizar el Program ID en 3 archivos

Con el ID del paso anterior:

**`program/programs/centralize/src/lib.rs`**
```rust
declare_id!("TU_PROGRAM_ID");
```

**`program/Anchor.toml`**
```toml
[programs.localnet]
centralize = "TU_PROGRAM_ID"
```

**`infrastructure/dev/.env`**
```env
PROGRAM_ID=TU_PROGRAM_ID
```

### 6. Recompilar con el ID correcto

```bash
anchor keys sync
anchor build
```

---

## Arranque — Cada vez que abres el proyecto

```bash
# 1. Levantar Docker (validator Solana + PostgreSQL + backend)
cd infrastructure/
make dev

# 2. Esperar ~15 segundos que el validator arranque

# 3. Cargar SOL al validator local
solana airdrop 10 --url http://localhost:8899
solana airdrop 10 --url http://localhost:8899
solana airdrop 10 --url http://localhost:8899

# 4. Desplegar el contrato
cd ../program/
anchor program deploy target/deploy/centralize.so --provider.cluster localnet
```

> ⚠️ **Importante:** Nunca uses `docker compose down` — el validator de Solana perderá el ledger y tendrás que redesplegar. Usa solo `docker compose restart backend`.

---

## Usar la consola API

Abre `frontend/marketplace-tester.html` con Live Server.

### Paso obligatorio después de cada deploy

Antes de cualquier otra operación, inicializa la plataforma **una sola vez**:

1. Ir al panel **⚠️ Initialize Platform** (naranja, arriba del sidebar)
2. Click en **Inicializar Plataforma**
3. Debes recibir una respuesta como esta:

```json
{
  "success": true,
  "tx_signature": "5N7da4sTVBZDJouWFK6fXC...EcEPhCA",
  "timestamp": "2026-04-22T18:21:00+00:00",
  "mensaje": "Plataforma inicializada"
}
```

Si la firma **no** empieza con `SIM-`, significa que se registró en la blockchain. ✅

### Registrar una empresa

| Campo | Valor de ejemplo |
|-------|-----------------|
| Nombre | `Acme S.A. de C.V.` |
| RFC | `ACM0101011AA` (exactamente 12 caracteres) |
| Categoría | `Tecnología` |
| Hash Doc. Constitución | `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa` (64 hex) |

---

## Flujo completo

```
1. initialize_platform     ← una vez por deploy
2. register_empresa        ← registra empresa (ID: 0, 1, 2...)
3. register_proveedor      ← registra proveedor (ID: 0, 1, 2...)
4. activar_membresia_*     ← admin activa membresía tras pago
5. iniciar_auditoria       ← empresa audita proveedor
6. concluir_auditoria      ← empresa registra resultado
7. calificar_proveedor     ← empresa califica al proveedor
8. publicar_solicitud      ← empresa publica licitación
9. asignar_solicitud       ← empresa asigna a proveedor
10. cerrar_solicitud       ← empresa cierra la licitación
```

---

## Roadmap

Esta es la **Etapa 1** — identidad y registro on-chain.

| Etapa | Estado | Descripción |
|-------|--------|-------------|
| 1 — Registro | ✅ Actual | Empresas, proveedores, membresías, auditorías, calificaciones, licitaciones |
| 2 — Documentos | 🔜 Planeado | Firma digital de contratos y acuerdos | zero trust 
| 3 — Pagos | 🔜 Planeado | Pagos entre empresas y proveedores on-chain |

---

## Estructura del repositorio

```
centrilze/
├── backend/          # API REST en Rust (Axum)
├── frontend/         # Consola HTML para pruebas
├── infrastructure/   # Docker Compose, .env
└── program/          # Smart contract Anchor/Rust
    └── programs/
        └── centralize/
            └── src/
                ├── lib.rs
                ├── errors.rs
                ├── instructions/
                └── state/
```

---

## Notas importantes

- El archivo `target/deploy/centralize-keypair.json` contiene la llave privada del programa — **no lo subas a git**
- Agrega al `.gitignore`: `target/deploy/centralize-keypair.json`
- Los IDs de empresa y proveedor son secuenciales (0, 1, 2...) — el primero registrado siempre es `0`
- El hash de documentos debe ser SHA-256 en hex (64 caracteres)
