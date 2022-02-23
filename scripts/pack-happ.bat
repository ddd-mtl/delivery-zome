REM Compile the WASM
cargo build --release --target wasm32-unknown-unknown
REM Pack DNAs
hc dna pack --output=delivery.dna dna.workdir
REM Pack the Happ with everything
hc app pack --output=delivery.happ dna.workdir

REM test zome
hc dna pack --output=secret.dna test_dna\\dna
hc app pack --output=secret.happ test_dna\\dna