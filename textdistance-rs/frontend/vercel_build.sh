#!/bin/bash
# Vercel Build Script for Rust WASM + Vite

# 1. Install rustup and target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown

# 2. Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 3. Build the WASM in the root directory
cd ..
wasm-pack build --target web

# 4. Build the Vite React app
cd frontend
npm run build
