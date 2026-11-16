# Learnings

One line per learning. Format: `YYYY-MM-DD [domain] avoid X, do Y, because Z`.

## Confirmed

- 2026-11-07 [naming] avoid PascalCase or camelCase frontend source filenames; use kebab-case and preserve generator-owned binding filenames, because consistent file paths are easier to scan and prevent naming drift.

## Inbox

- 2026-11-16 [rust] avoid `#[strum(serialize = "...")]` on variants whose string equals the variant name, do rely on the strum default or `serialize_all`, because the attribute is redundant noise and only the variants that differ need an override.
