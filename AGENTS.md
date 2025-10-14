# Repository Guidelines

## Project Structure & Module Organization
This monorepo anchors Rust services in `codex-rs`, a Cargo workspace where each crate is prefixed with `codex-` (for example, `codex-core`, `codex-tui`). CLI tooling for agents lives in `codex-cli`, while shared automation scripts sit in `commands/` and `scripts/`. Markdown documentation is gathered under `docs/`, and JavaScript utilities reside at the root alongside `package.json`. Tests generally live beside their crates in `codex-rs/<crate>/tests`, with snapshot fixtures stored in `codex-rs/tui/tests/snapshots`.

## Build, Test, and Development Commands
Install required Rust tooling via `just install` in `codex-rs`. Format Rust code with `just fmt`, and scope lint fixes using `just fix -p codex-tui` (swap the crate name as needed). Use `cargo nextest run --no-fail-fast` or the shortcut `just test` for broad coverage, while targeted checks rely on `cargo test -p codex-core` and similar. For TypeScript utilities, run `pnpm install` once and `pnpm format` to keep JSON and Markdown consistent.

## Coding Style & Naming Conventions
Rust code follows the repository `rustfmt.toml` plus Clippy rules: collapse nested `if` statements, inline `format!` arguments, and favor method references over redundant closures. TUI code should use the Stylize helpers described in `codex-rs/tui/styles.md` (e.g., `"label".cyan().bold()`). Snapshot tests must call `pretty_assertions::assert_eq`, and crates, binaries, and modules keep the `codex-*` prefix to stay aligned with workspace expectations.

## Testing Guidelines
Prefer `cargo nextest` for speed; fall back to `cargo test` when adding new suites. Regenerate terminal snapshots with `cargo test -p codex-tui`, inspect deltas via `cargo insta pending-snapshots -p codex-tui`, and accept intentional updates with `cargo insta accept -p codex-tui`. Integration suites in `codex-rs/core` should reuse `core_test_support::responses` helpers to assert SSE traffic cleanly.

## Commit & Pull Request Guidelines
Commit messages follow a Conventional Commit flavor (`feat:`, `fix:`, etc.) as seen in recent history. Keep each pull request focussed, include a brief summary of scope, testing evidence (`cargo test -p ...` outputs), and link to any related issues. When UI or protocol surfaces change, attach screenshots or schema diffs so reviewers can verify agent-facing impact.

## Security & Configuration Notes
Respect sandbox variables such as `CODEX_SANDBOX_NETWORK_DISABLED`; tests often short-circuit when they detect them. Avoid introducing new secrets into configuration files, and prefer existing helpers in `codex-rs/config.md` for environment management.
