# Derbe

Derbe is an employee leave management application. The backend uses Rust, Axum and PostgreSQL. The frontend uses React, TypeScript, Vite, TanStack Query and shadcn/ui.

## Prerequisites

The following tools are required for local development:

| Tool | Version / purpose |
|---|---|
| Rust | Stable toolchain |
| Node.js | 26 LTS |
| pnpm | 12.5.1 |
| Docker Desktop | Local PostgreSQL database |
| cargo-watch | Automatically rebuilds and restarts the Rust backend |
| SQLx CLI | Runs database migrations |

### 1. Install Rust

Install Rust using [rustup](https://rustup.rs/).

Verify the installation:

```sh
rustc --version
cargo --version
```

Ensure you are using the stable toolchain:

```sh
rustup default stable
```

### 2. Install Node.js using nvm

Using [nvm](https://github.com/nvm-sh/nvm) is recommended to install and manage Node.js versions.

On macOS or Linux, install nvm:

```sh
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.8/install.sh | bash
```

Load nvm into your current terminal session (or restart your terminal):

```sh
. "$HOME/.nvm/nvm.sh"
```

Install Node.js 26 LTS:

```sh
nvm install 26
```

Verify the installation:

```sh
node -v
npm -v
```

### 3. Install pnpm

Derbe uses pnpm 12.5.1, specified in the root `package.json`.

Install Corepack:

```sh
npm install -g corepack
```

Enable pnpm:

```sh
corepack enable pnpm
```

Activate the version required by Derbe:

```sh
corepack prepare pnpm@12.5.1 --activate
```

Verify:

```sh
pnpm -v
```

The expected version is `12.5.1`.

### 4. Install Rust development utilities

Install `cargo-watch` and the SQLx CLI:

```sh
cargo install cargo-watch
cargo install sqlx-cli
```

`cargo-watch` monitors Rust source files and automatically rebuilds and restarts the backend when changes are detected.

The SQLx CLI is used to apply database migrations.

### 5. Install Docker Desktop

Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) and start it.

Verify Docker is running:

```sh
docker --version
docker ps
```

## Local development setup

### 1. Clone the repository

```sh
git clone https://github.com/bfedward/derbe.git
cd derbe
git checkout main
```

### 2. Start PostgreSQL

Derbe uses PostgreSQL for persistent storage. The following command creates a local development database using Docker:

```sh
docker run -d --name derbe-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=derbe \
  -p 5432:5432 postgres
```

This creates a database with the following connection details:

| Setting | Value |
|---|---|
| Host | 127.0.0.1 |
| Port | 5432 |
| Database | derbe |
| Username | postgres |
| Password | password |

These are local development credentials only. Do not use them in production.

**Note:** You only need to create the container once. On subsequent runs, start the existing container with:

```sh
docker start derbe-postgres
```

If it is already running, no action is necessary.

### 3. Apply database migrations

Navigate to the backend directory:

```sh
cd derbe-backend
```

Run the SQLx migrations:

```sh
sqlx migrate run
```

This applies any outstanding migrations and creates the database tables.

The backend's local database configuration is provided by `derbe_config.yaml` and `.env`.

If Rust Analyzer reports SQLx errors indicating that tables do not exist after running migrations, try restarting Rust Analyzer so it picks up the updated database schema.

Return to the repository root:

```sh
cd ..
```

### 4. Install frontend dependencies

From the repository root:

```sh
pnpm install
```

This installs the JavaScript and TypeScript dependencies for the frontend and development tooling.

### 5. Start Derbe

Make sure Docker is running and PostgreSQL is available, then run:

```sh
pnpm dev
```

This starts the development environment:

- Waits for PostgreSQL to become available on port 5432.
- Starts the Axum backend using `cargo-watch`.
- Waits for the backend to become available on port 8001.
- Starts the Vite frontend.

Open the application in your browser:

**http://localhost:4200**

The backend API listens on:

**http://127.0.0.1:8001**

### Running the services independently

You can also start the frontend and backend separately.

| Command | Description |
|---|---|
| `pnpm dev` | Starts the full application |
| `pnpm dev:backend` | Starts the Rust backend |
| `pnpm dev:frontend` | Starts the Vite frontend |

The frontend requires the backend to be running for API functionality.

### Frontend configuration

The frontend uses `VITE_API_BASE_URL` to configure the backend API address.

By default, the local backend listens on port 8001.

To use a different backend address, configure:

```sh
VITE_API_BASE_URL=http://127.0.0.1:8001
```

The backend port can be changed using `DERBE_APPLICATION__APPLICATION_PORT`.

## Frontend development

Run these commands from the repository root.

| Command | Description |
|---|---|
| `pnpm typecheck` | Checks TypeScript types |
| `pnpm lint` | Runs Biome checks |
| `pnpm knip` | Finds unused files and dependencies |
| `pnpm build` | Builds the production frontend |
| `pnpm check:quality` | Runs typecheck, lint and knip |

Run the quality checks before submitting frontend changes:

```sh
pnpm check:quality
pnpm build
```

### Exporting TypeScript bindings

Rust DTOs and enums are the source of truth for API contracts.

After changing Rust DTOs or enums, regenerate the TypeScript bindings:

```sh
cd derbe-backend
cargo run --bin export_bindings
```

Generated TypeScript declarations live in:

```text
derbe-frontend/src/bindings
```

Do not edit generated binding files manually.

## Architecture

```text
React pages
    |
TanStack Query
    |
TypeScript API client
    |
Axum API
    |
PostgreSQL

Rust DTOs and enums
    |
    ts-rs
    |
Generated TypeScript bindings
```