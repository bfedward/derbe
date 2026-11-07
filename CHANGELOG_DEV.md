# Dev Changelog

Internal development history for Derbe. Not user-facing.

## [Unreleased]

## [0.2.0] - 2026-11-07

### Frontend naming and structure

- Renamed frontend source files to kebab-case, including pages, forms, and shared components. Updated imports and page route declarations. Generated `src/bindings` filenames remain generator-owned and unchanged.
- Replaced ambiguous generic `status-badge.tsx` with `leave-status-badge.tsx` and made leave-status labels/colors explicit in a typed map.
- Added `AGENTS.md`, `CLAUDE.md`, and `__LEARNING/LEARNINGS.md`; recorded kebab-case and generated-binding naming as a confirmed learning.

### Backend architecture and behavior

- Grouped each backend feature under `derbe-backend/src/features/`, keeping its handlers, request/response DTOs, model, repository, and service together. Root `routes.rs` composes feature handlers. No `pub use` compatibility facades.
- Preserved existing HTTP route paths, methods, JSON shapes, error messages, and status codes, including existing `201 Created` create responses.
- Replaced controller terminology with handlers, renamed generic CRUD methods to `get_by_id`, `list`, `create`, `update`, `delete`, and validation methods to intent-revealing names.
- Added `AppState`; made database/listener/server startup errors fallible with `thiserror`. Replaced `println!` with `tracing` while keeping existing error response text.
- Used `strum` for enum display and parsing. Added fallible `HalfDayOptions::try_from_string`; kept `from_string` and tuple-based conversion signatures compatible. External serialized labels stay unchanged.
- Simplified SQLx error propagation with `?`, kept typed `query_as!`/`query_scalar!` queries, and made deletes ignore missing rows as before.
- Synchronized backend and frontend package versions at `0.2.0`. Release tags use annotated `vMAJOR.MINOR.PATCH` tags.

Before, handler code repeated result matching:

```rust
match service_call().await {
    Ok(value) => Ok(Json(value)),
    Err(error) => Err(error),
}
```

After, `?` propagates errors directly:

```rust
let value = service_call().await?;
Ok(Json(value))
```

Before, feature files were split by technical layer:

```text
src/controller/
src/service/
src/repository/
src/model/
src/dto/
```

After, each feature owns its implementation:

```text
src/features/employees/
  dto.rs
  dto/request.rs
  dto/response.rs
  handlers.rs
  mod.rs
  model.rs
  repository.rs
  service.rs
```

## Versioning

Use Semantic Versioning. Keep `derbe-backend/Cargo.toml` and `derbe-frontend/package.json` versions synchronized. Release tags use `vMAJOR.MINOR.PATCH`.
