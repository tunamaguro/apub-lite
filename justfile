_default:
  just --list 


set export
set dotenv-load


export RUST_BACKTRACE := "1"

alias f:= format
alias l:= lint
alias r:= ready


# Install tools
install-tools:
    cargo install cargo-binstall
    cargo binstall cargo-watch

# Format `.rs` files
format:
    cargo fmt

# Run clippy
lint:
    cargo clippy  --all-targets --all-features

# Run tests
test:
    cargo test

# Check code is ready
ready: 
    just format
    just lint
    just test

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


SERVEO_ADDR := "serveo_addr.txt"
SERVEO_PID := "serveo_pid.txt"
    
# Start serveo 
[unix]
start_serveo:
    #!/usr/bin/bash
    ssh -R 80:localhost:3000 serveo.net > ${SERVEO_ADDR} 2>&1 &
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