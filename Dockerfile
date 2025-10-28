# ----------------------------
# Builder
# ----------------------------
FROM rust:1.82 as builder

WORKDIR /app

# Copy entire workspace (Cargo needs the workspace root to resolve paths)
COPY . .

# Build release binaries
RUN cargo build --release --workspace

# ----------------------------
# Final stage: server
# ----------------------------
FROM debian:bookworm-slim as server
WORKDIR /app

# Copy server binary only
COPY --from=builder /app/target/release/server /usr/local/bin/server
CMD ["server"]

# ----------------------------
# Final stage: client
# ----------------------------

# Copy client binary only
FROM debian:bookworm-slim as client
WORKDIR /app

# Install OpenSSL runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/eleventh /usr/local/bin/eleventh
CMD ["eleventh"]
