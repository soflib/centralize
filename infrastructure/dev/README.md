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