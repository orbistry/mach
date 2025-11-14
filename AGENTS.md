# Repository Guidelines

## Project Structure & Product Context

- Workspace is pure Ratatui; `crates/mach` replaces Tweek’s weekly calendar
  (Mon–Sun columns plus Someday/backlog). Ignore legacy Tauri/Svelte references.
- Keep UI widgets under `crates/mach/src/ui/` and persistence/state
  orchestration under `src/services/` so weekly lanes evolve independently.
- Add dependencies to `[workspace.dependencies]` first, then reference them from
  member crates to maintain version lockstep.

## Build, Test, and Development Commands

- `cargo run -p mach` (add `--release` when profiling) renders the Ratatui UI;
  aim for visual parity with the Tweek reference.
- `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`,
  and `cargo test --workspace -- --nocapture` gate every commit.

## Database Stack (SeaORM 2 + Turso)

- Use Turso’s embedded builder
  (`let db = Builder::new_local("mach.db").build().await?;`) for storage, then
  expose the same file via SeaORM (`Database::connect("sqlite://mach.db?mode=rwc")`
  or tuned `ConnectOptions` for pool limits, timeouts, and ping checks).
- SeaORM 2’s `#[sea_orm::model]` macro, `Entity::COLUMN.*` helpers, and richer
  `HasOne/HasMany` wrappers (now distinguish `Unloaded`, `NotFound`, `Loaded`)
  keep queries type-safe; hydrate daily, Someday, and reference buckets via `.with(...)`.
- Enable the Entity-first workflow (`schema-sync` + `entity-registry`) and call
  `db.get_schema_registry("mach::entity::*").sync(db).await?` so schema diffs
  apply automatically in dependency order.
- Model task metadata (`labels`, `links`, `scheduled_for`) as typed columns to
  leverage compile-time `contains`, `like`, and numeric comparison helpers.

## Coding Style & Naming Conventions

- Four-space indentation, `snake_case` modules, `PascalCase` types,
 `SCREAMING_SNAKE_CASE` constants. Keep functions ~50 LOC and split
  widgets/services as they grow.
- Co-locate SeaORM entities with their feature modules, re-export via
  `mod.rs`, and add terse headers for domain-heavy files (e.g., Someday lane rules).
- Use `miette` for diagnostics and user-facing errors; avoid `anyhow` so
  all contexts surface with consistent, nicely formatted reports.

## Testing Guidelines

- Inline unit tests stay under `#[cfg(test)]`; integration tests live in
  `crates/mach/tests/` (`weekly_board.rs`, `someday_sync.rs`, etc.).
- Use `tokio::test` with Turso in-memory (`Builder::new_local(":memory:")`) to
  exercise SeaORM queries and verify `schema-sync` idempotence.
- Assert `HasOne/HasMany` states (Unloaded vs NotFound) and keep >80% coverage
  across reducers, filters, and scheduling logic before requesting review.
