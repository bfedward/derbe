# derbe
derbe is a HR tool that allows employees to manage their leave.
derbe is made with Yew, Axum and PostgreSQL.

# Local Developer Setup

## Backend

Rust needs to be installed. See [this page](https://www.rust-lang.org/learn/get-started) for instructions to install Rust using rustup for your OS.

derbe needs a connection to a PostgreSQL database to store data. This connection is needed during development as sqlx is used to verify database queries.
Run this command in a terminal to create a Docker container using the offical PostgreSQL image:

`docker run -d \
  --name postgres-container \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=derbe \
  -p 5432:5432 \
  postgres
`

To run all the database migrations and create new migrations, the [sqlx cli](https://crates.io/crates/sqlx-cli) is needed. 
In a terminal window, run this command:
`cargo install sqlx-cli`

This will install the sqlx cli with all its dependencies. It does not need to be re-installed for every new Rust project.

Now run `sqlx migrate run`. The command output will show all database migrations being run in the order they appear in derbe-backend/migrations.

The backend can now be run by entering `cargo run`. Cargo will download all dependencies, compile everything into an executable and run it.

## Frontend

The derbe frontend is made using Yew, which has a couple of [setup steps](https://yew.rs/docs/getting-started/introduction).

Yew needs the compilation target for browser-based WebAssembly (WASM). 
To add the compilation target, run this:
`rustup target add wasm32-unknown-unknown`

To build and run a WASM project, trunk needs to be installed:
`cargo install trunk`

After trunk has finished installing, the frontend can be run:
`trunk serve`

derbe will then be running on [localhost:4200](localhost:4200)