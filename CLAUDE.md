# ratatui-tasks

Rust/ratatui task TUI (`src/main.rs`, `src/bin`). Not the dayagenda app (`~/developer/dayagenda`).

**Superseded 2026-10-05 by dayagenda's Life view.** This app is broken as it stands:
`DB_PATH` points at `~/Claude Desktop/personal-tasks/tasks-db.md`, which does not exist,
and its toggle rewrites that markdown mirror in place, a file life-admin says is never
hand-edited. dayagenda reads `~/cowork/life-admin/db/tasks.db` read-only instead. Kept
for its `inline` viewport example; do not build on it.

## Stack
Rust 2024 edition, ratatui 0.30, crossterm, color-eyre. `cargo run`.

## Do not
- Commit `target/`.
