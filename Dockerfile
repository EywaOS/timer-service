# ... (parte iniziale)
WORKDIR /usr/src/app

# Copia i manifesti
COPY Cargo.toml Cargo.lock ./

# --- AGGIUNGI QUESTO ---
# Devi copiare il codice sorgente prima di compilare!
COPY src ./src
# Se ti servono le cartelle config o migration durante la build (es. sqlx), copiale qui:
# COPY config ./config 
# COPY migration ./migration
# -----------------------

# Ora puoi compilare
RUN cargo build --release --bin timer-service

# ... (resto del dockerfile)
