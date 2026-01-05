# Stage 1: Builder
FROM rust:1.85 as builder

WORKDIR /usr/src/app

# 1. Creiamo un progetto vuoto per cachare le dipendenze
RUN USER=root cargo new --bin timer-service
WORKDIR /usr/src/app/timer-service

# 2. Copiamo SOLO il Cargo.toml (Rimosso Cargo.lock perché non esiste ancora)
COPY Cargo.toml ./

# 3. Questo build scarica le dipendenze, genera il lockfile e compila
RUN cargo build --release
RUN rm src/*.rs

# 4. Ora copiamo il TUO codice sorgente vero
COPY src ./src

# 5. Compiliamo il binario vero
# Rimuoviamo il "fingerprint" per forzare la ricompilazione
RUN rm ./target/release/deps/timer_service*
RUN cargo build --release --bin timer-service

# Stage 2: Runtime
FROM debian:bookworm-slim

# Installiamo le dipendenze runtime
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copiamo il binario dal builder
COPY --from=builder /usr/src/app/timer-service/target/release/timer-service .

# Copiamo le cartelle di configurazione (se esistono nel repo)
COPY config ./config
COPY migration ./migration

# Lancia il servizio
CMD ["./timer-service"]
