default:
    just build

build:
    cargo build --workspace

sb:
    cd backend && cargo watch --ignore "src/tests/" -x "shuttle run"

cb:
    cd backend && cargo check

sf:
    cd frontend && trunk serve --port 3000

cf:
    cd frontend && cargo check

sa:
    just sb & just sf

