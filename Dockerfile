# Stage 1: Builder
FROM rust:1.85 as builder

WORKDIR /usr/src/app

# 1. Creiamo un progetto vuoto
RUN USER=root cargo new --bin timer-service 
WORKDIR /usr/src/app/timer-service

# 2. Copiamo il manifesto E la dipendenza locale 'migration'
COPY Cargo.toml ./
# --- CORREZIONE FONDAMENTALE ---
# Poiché Cargo.toml ha una riga tipo "timer-service-migration = { path = 'migration' }",
# dobbiamo copiare questa cartella ORA, altrimenti il build delle dipendenze fallisce.
COPY migration ./migration

# 3. Questo build scarica le dipendenze e compila anche il crate 'migration'
RUN cargo build --release
RUN rm src/*.rs

# 4. Ora copiamo il codice sorgente del servizio principale
COPY src ./src

# 5. Compiliamo il binario finale
# Rimuoviamo il fingerprint per forzare la ricompilazione del main
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

# Copiamo le cartelle di configurazione necessarie a runtime
COPY config ./config
COPY migration ./migration

# Lancia il servizio
CMD ["./timer-service"]
