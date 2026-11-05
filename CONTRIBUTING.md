# Contributing

Keep changes focused and follow the existing Rustify frontend patterns where they fit Derbe.

## Frontend

- Use TypeScript strict mode and the generated contracts in `derbe-frontend/src/bindings`.
- Keep server state in TanStack Query. Define query keys in `src/query-keys.ts` and reusable query options in `src/api.ts`.
- Use shadcn/ui primitives from `src/components/ui` and shared class merging through `cn`.
- Regenerate bindings with `cd derbe-backend && cargo run --bin export_bindings` after contract changes. Never hand-edit generated bindings.
- Run `pnpm check:quality` and `pnpm build` before submitting frontend changes.

## Backend

- Keep API and database behavior unchanged when a frontend-only change can solve the requirement.
- Keep Rust DTOs and enums as the source of truth for TypeScript API contracts.
- Run `cargo check` from `derbe-backend` after backend changes.
