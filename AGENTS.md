# Agent instructions

## Project

- Derbe is a Rust, Axum, SQLx and PostgreSQL backend with a React, TypeScript, Vite frontend.
- Read `README.md` for setup and development commands.
- Keep backend changes within the existing `derbe-backend` structure and frontend changes within `derbe-frontend/src` unless the task requires otherwise.
- Use pnpm for JavaScript and TypeScript commands.
- Use kebab-case filenames for frontend source files; keep generated `src/bindings` filenames as produced by `ts-rs`. Keep React component names in PascalCase.
- Match nearby code conventions before introducing new patterns or dependencies.
- Never add secrets or production credentials to source control.

## Learning workflow

- Capture only recurring guidance that can improve future work. Skip one-off fixes and rules already enforced by tooling.
- Add each learning as one line under `Inbox` in `__LEARNING/LEARNINGS.md` using `YYYY-MM-DD [domain] avoid X, do Y, because Z`.
- Record concrete avoid/prefer behavior and why it matters. Keep entries pending in `Inbox`; move to `Confirmed` only after the user validates them.
- If the user says `drop it`, remove the learning they refer to from the inbox.
- Do not edit confirmed learnings without the user's direction.
