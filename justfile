_default:
  just --list 


set dotenv-load


export RUST_BACKTRACE := "1"

alias f:= format
alias l:= lint
alias lf:= lint-fix
alias r:= ready


# Install tools
install:
    cargo install cargo-binstall 
    cargo binstall cargo-watch taplo-cli cargo-nextest sqlx-cli sqruff

# Format `.rs` files
format:
    cargo fmt
    taplo format
    sqruff fix --force

# Run clippy
lint:
    cargo clippy --all-targets --all-features --fix

# Fix clippy errors if dirty or staged
lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# Fix clippy error
fix:
    cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged

# Run all tests
test:
    cargo nextest run --workspace

# Check code is ready
ready: 
    just format
    just lint
    just test

# Find unused deps
udeps:
    cargo install cargo-udeps --locked
    rustup install nightly
    cargo +nightly udeps --workspace --all-targets

# Start dev server
[unix]
dev:
    #!/usr/bin/bash
    if [ -z "${APUB_LITE_URL}" ]; then
        just start_serveo
        export APUB_LITE_URL=$(cat ${SERVEO_ADDR})
        SERVEO_STARTED=1
    else
        SERVEO_STARTED=0
    fi
    if [ "$SERVEO_STARTED" -eq 1 ]; then
        trap "just finish_serveo" EXIT
    fi
    cargo watch -x run


export SERVEO_ADDR := "serveo_addr.txt"
export SERVEO_PID := "serveo_pid.txt"
    
# Start serveo 
[unix]
start_serveo:
    #!/usr/bin/bash
    ssh -R 80:localhost:8080 serveo.net > ${SERVEO_ADDR} 2>&1 &
    echo $! > ${SERVEO_PID}

    while true; do
        if grep -q 'Forwarding HTTP traffic from' ${SERVEO_ADDR}; then
            ADDR=$(grep 'Forwarding HTTP traffic from' ${SERVEO_ADDR} | grep -o 'https://.*serveo.net')
            echo "HTTP forwarded from $ADDR"
            echo $ADDR > $SERVEO_ADDR
            break
        fi
        sleep 1
    done
    

# Kill serveo ssh and remove files
[unix]
finish_serveo:
    #!/usr/bin/bash
    if [ -f ${SERVEO_PID} ]; then
        kill $(cat ${SERVEO_PID})
        rm ${SERVEO_PID}
        echo "serveo finished"
    else
        echo "serveo is not running"
    fi
    if [ -f ${SERVEO_ADDR} ]; then
        rm ${SERVEO_ADDR}
    fi

MIGRATION_DIR := "crates/apub-adapter/migrations"

# Add migrate file
migrate name:
    cargo sqlx migrate add -r {{name}} --source {{MIGRATION_DIR}}

# Apply migrate
apply-db:
    cargo sqlx migrate run --source {{MIGRATION_DIR}}

# Revert latest migrate
revert-db:
    cargo sqlx migrate revert --source {{MIGRATION_DIR}}

# Reset database
reset-db:
    cargo sqlx database drop
    cargo sqlx database create
    just apply-db