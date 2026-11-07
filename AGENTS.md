# Agent instructions

## Project

- Derbe is a Rust, Axum, SQLx and PostgreSQL backend with a React, TypeScript, Vite frontend.
- Read `README.md` for setup and development commands.
- Keep backend changes within the existing `derbe-backend` structure and frontend changes within `derbe-frontend/src` unless the task requires otherwise.
- Use pnpm for JavaScript and TypeScript commands.
- Before creating or renaming frontend files, check `__LEARNING/LEARNINGS.md`. Use kebab-case filenames for frontend source files; keep generated `src/bindings` filenames as produced by `ts-rs`. Keep React component names in PascalCase.
- Match nearby code conventions before introducing new patterns or dependencies.
- Never add secrets or production credentials to source control.
- Keep `CHANGELOG_DEV.md` current in the same commit for meaningful code, architecture, dependency, tooling, or agent-guidance changes. Add work under `[Unreleased]`; use concise before/after code blocks when they clarify a refactor.
- For every version upgrade, update both `derbe-backend/Cargo.toml` and `derbe-frontend/package.json`, move matching `CHANGELOG_DEV.md` entries into a dated version section, then create an annotated Git tag named `vMAJOR.MINOR.PATCH` at that release commit. Keep version, changelog, and tag in sync.
- Organize backend domain code by feature under `derbe-backend/src/features/<feature>/`; keep each feature's handlers, DTOs, model, repository, and service together. Avoid `pub use` re-export facades.

## Learning workflow

- Capture only recurring guidance that can improve future work. Skip one-off fixes and rules already enforced by tooling.
- Add each learning as one line under `Inbox` in `__LEARNING/LEARNINGS.md` using `YYYY-MM-DD [domain] avoid X, do Y, because Z`.
- Record concrete avoid/prefer behavior and why it matters. Keep entries pending in `Inbox`; move to `Confirmed` only after the user validates them.
- If the user says `drop it`, remove the learning they refer to from the inbox.
- Do not edit confirmed learnings without the user's direction.
