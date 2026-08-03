# Stage 1: Build the React frontend
FROM node:20-slim AS frontend-builder
WORKDIR /app
COPY textdistance-rs/frontend/package*.json ./
RUN npm install
COPY textdistance-rs/frontend/ ./
RUN npm run build

# Stage 2: Build the Rust backend
FROM rust:1.82-slim AS backend-builder
WORKDIR /app
COPY textdistance-rs/Cargo.toml textdistance-rs/Cargo.lock* ./
COPY textdistance-rs/src/ src/
RUN cargo build --release

# Stage 3: Final runtime image
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=backend-builder /app/target/release/textdistance /usr/local/bin/textdistance
COPY --from=frontend-builder /app/dist /app/frontend/dist

# Expose port for the SaaS API
EXPOSE 10000

# Start the SaaS API server (reads PORT env var automatically)
CMD ["textdistance", "--serve"]
