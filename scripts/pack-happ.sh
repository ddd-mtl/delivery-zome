#!/bin/bash

set -e

# Compile the WASM
#cargo build --release --target wasm32-unknown-unknown
# test zome
hc dna pack --output=secret.dna playground/workdir
hc app pack --output=secret.happ playground/workdir
