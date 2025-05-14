default:
    just build

build:
    cargo build --workspace

serve-backend:
    cd backend && cargo shuttle run

serve-frontend:
    cd frontend && trunk serve --port 3000

serve:
    just serve-backend & just serve-frontend
