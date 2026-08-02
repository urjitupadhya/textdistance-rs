# Stage 1: Build the React frontend
FROM node:20-slim AS frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ ./
RUN npm run build

# Stage 2: Build the Rust backend
FROM rust:1.82-slim AS backend-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/
RUN cargo build --release

# Stage 3: Final runtime image
FROM debian:bookworm-slim
# Install Python for differential fuzzing (optional, but good for hackathon completeness)
RUN apt-get update && apt-get install -y python3 python3-pip && rm -rf /var/lib/apt/lists/*
RUN pip3 install --break-system-packages textdistance hypothesis

WORKDIR /app
COPY --from=backend-builder /app/target/release/textdistance /usr/local/bin/textdistance
COPY --from=frontend-builder /app/dist /app/frontend/dist

COPY tests/ tests/
COPY fuzz/ fuzz/
COPY bench/ bench/
COPY DECISIONS.md README.md .port-mortem.toml ./

# Expose port for the SaaS API
EXPOSE 3000

# Start the SaaS API server
CMD ["textdistance", "--serve", "--port", "3000"]
