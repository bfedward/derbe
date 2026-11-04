# Derbe

Derbe is an employee leave management tool. The backend uses Rust, Axum, and PostgreSQL. The frontend uses React, TypeScript, Vite, TanStack Query, and shadcn/ui.

## Requirements

- Rust stable and `cargo-watch`
- Node.js and pnpm 12
- Docker with Compose
- SQLx CLI

## Local setup

Start a PostgreSQL database for Derbe. For a local Docker instance:

```sh
docker run -d --name derbe-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=derbe \
  -p 5432:5432 postgres
```

Apply database migrations:

```sh
cd derbe-backend
cargo install sqlx-cli
sqlx migrate run
cd ..
```

Install JavaScript dependencies and start the full stack:

```sh
pnpm install
pnpm dev
```

Frontend runs at <http://localhost:4200>. API listens at <http://127.0.0.1:8001>.
Set `DERBE_APPLICATION__APPLICATION_PORT` to override the API port, or
`VITE_API_BASE_URL` to point the frontend at another API address.

Run frontend commands from the repository root:

```sh
pnpm typecheck
pnpm lint
pnpm knip
pnpm build
```

Export Rust API types after changing DTOs or enums:

```sh
cd derbe-backend
cargo run --bin export_bindings
```

Generated TypeScript declarations live in `derbe-frontend/src/bindings`. Do not edit these files manually.

## Architecture

```text
React pages -> TanStack Query -> TypeScript API client -> Axum API -> PostgreSQL
                                      ^
                                      |
                            generated ts-rs bindings
```
