REFERCIAN E INTRO A LA PRIMER ETAPA DEL PROYECTO
centralize-intro.html

en .env.example viene el ejemplo del .env para infrastructure/dev


# 1. Instalar Solana en PATH
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc

**********************************************************************************************
# 2. Keypair de tu wallet (si ya existe, sáltalo)
solana-keygen new --outfile ~/.config/solana/id.json --no-bip39-passphrase
solana address
# si ya existe: solo corre → solana address

solana config set --url http://localhost:8899

**********************************************************************************************
# 3. Primera compilación para crear la carpeta target/deploy/
cd "/program"
anchor build

**********************************************************************************************
# 4. Generar keypair FIJO del programa (sobreescribe el que generó anchor)
solana-keygen new --outfile target/deploy/centralize-keypair.json --no-bip39-passphrase
if exists: solana-keygen pubkey target/deploy/centralize-keypair.json

**********************************************************************************************
# 5. Ver el Program ID — ANÓTALO, es permanente
solana-keygen pubkey target/deploy/centralize-keypair.json

**********************************************************************************************
# 6. Poner ese PROGRAM_ID en los 3 archivos:
#    a) programs/centralize/src/lib.rs  → declare_id!("PROGRAM_ID")
#    b) Anchor.toml → [programs.localnet] centralize = "PROGRAM_ID"
#    c) infrastructure/.env → PROGRAM_ID=...

**********************************************************************************************
# 7. Recompilar con el ID correcto
anchor keys sync
anchor build

**********************************************************************************************
# 8. Levantar Docker
cd "/infrastructure"
make dev

**********************************************************************************************
# 9. Esperar 15 segundos que el validator arranque, luego airdrop
solana airdrop 10 --url http://localhost:8899

**********************************************************************************************
# 10. Desplegar el contrato
cd "/program"
anchor program deploy target/deploy/centralize.so --provider.cluster localnet

**********************************************************************************************
# 11. Abrir el HTML y probar(Open with live server)



en el frontend:

1. correr initialize_platform para generar
{ 
    "success": true, 
    "tx_signature": "..............................EcEPhCA", 
    "timestamp": "2026-04-22T18: 21: 00.603110073+00: 00", 
    "mensaje": "Plataforma inicializada" 
}

para llenar un register_empresa:
Nombre y Categoria (cualquiera)
para:
RFC
ACM0101011AA (12 chars)
Hash Doc. Constitución (64 hex)
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa