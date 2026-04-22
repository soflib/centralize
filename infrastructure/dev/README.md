# CertChain + TicketChain — Stack local Solana

## Requisitos
- Docker >= 24.x
- Docker Compose >= 2.x
- `openssl` (para generar swarm.key)

## Primer arranque

```bash
# 1. Clonar / entrar al proyecto
cd certchain

# 2. Crear archivo de secrets
cp .env.example .env
# Editar .env y cambiar POSTGRES_PASSWORD

# 3. Generar swarm key para IPFS privado
bash scripts/gen-swarm-key.sh

# 4. Levantar todo
docker-compose up -d

# 5. Ver logs
docker-compose logs -f
```

## Servicios y puertos

| Servicio | Puerto | Descripción |
|---|---|---|
| solana-validator | 8899 | RPC HTTP |
| solana-validator | 8900 | RPC WebSocket |
| postgres | 5432 | PostgreSQL (solo 127.0.0.1) |
| ipfs | 5001 | API IPFS (solo 127.0.0.1) |
| ipfs | 8082 | Gateway IPFS |

## Desplegar programas

Coloca tus archivos `.so` compilados en `./programs/`:
```
programs/
  certs.so
  tickets.so
```

Luego reinicia el deployer:
```bash
docker-compose up program-deployer
```

## Verificar estado
```bash
bash scripts/check-deploy.sh
```

## IPs fijas internas (entre contenedores)

| Servicio | IP |
|---|---|
| solana-validator | 172.28.0.10 |
| program-deployer | 172.28.0.11 |
| postgres | 172.28.0.20 |
| ipfs | 172.28.0.30 |
