# AGENTS.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Quick project facts
- Rust workspace (edition 2024) with MSRV **Rust 1.87** (see root `Cargo.toml`).
- Primary crate is `crates/gen-changelog/` which contains:
  - Library crate: `gen_changelog` (`crates/gen-changelog/src/lib/lib.rs`)
  - Binary crate: `gen-changelog` CLI (`crates/gen-changelog/src/cli/main.rs`)
- Task runner: `just` via root `justfile`.

## Common commands
Most day-to-day tasks are wrapped by `just` (see `justfile`). To see what’s available:
- `just --list`

If you don’t have `just`, run the equivalent `cargo …` commands shown below.

### Build / check
- Build all workspace crates:
  - `cargo build --workspace`
- Typecheck quickly:
  - `just check`
  - equivalent: `cargo check --all`

### Run the CLI locally
- Run CLI from workspace (recommended while developing):
  - `cargo run -p gen-changelog --bin gen-changelog -- --help`
  - example generation: `cargo run -p gen-changelog --bin gen-changelog -- generate --show`

- Install a dev build of the CLI into your Cargo bin dir (useful for scripts/hooks that expect `gen-changelog` on PATH):
  - `cargo install --path crates/gen-changelog --locked --force`

### Tests
- Full test suite (clippy + check + docs + unit tests):
  - `just test`
- Unit tests only:
  - `just unit-tests`
  - equivalent: `cargo test --all`

Run a single test by name (library unit tests):
- `cargo test -p gen-changelog <test_name_substring>`

Run the CLI integration tests (trycmd):
- `cargo test -p gen-changelog --test cli_tests`

### Lint / format
- Clippy:
  - `just clippy`
  - equivalent: `cargo clippy`

- Formatting (uses **nightly** rustfmt features, then verifies stable formatting, then formats the `justfile`):
  - `just fmt`

### Docs
- Build docs and fail on warnings:
  - `just doc`
  - equivalent: `RUSTDOCFLAGS="-D warnings" cargo doc --all --no-deps`

### Security / license audit
- Cargo-deny checks (advisories/bans/licenses/sources):
  - `just audit`
  - equivalent: `cargo deny check advisories bans licenses sources`

### Coverage
- Generate coverage report (requires `cargo-tarpaulin`):
  - `just cov`

## High-level architecture
### Workspace layout
- Root is a thin workspace wrapper (`Cargo.toml`) pointing at `crates/gen-changelog`.
- `docs/` contains the long-form docs that are embedded/assembled into published documentation:
  - `crates/gen-changelog/src/lib/lib.rs` includes `docs/lib.md` as crate-level docs.
  - Root `release-hook.sh` rebuilds `README.md` from `docs/readme/*` + `docs/*.md`.

### CLI flow (`crates/gen-changelog/src/cli/*`)
- `main.rs` is a small clap entrypoint:
  - Parses `generate` and `config` subcommands.
  - Sets up logging via `clap_verbosity_flag` + `env_logger`.
- `generate_cli.rs`:
  - Opens a git repo via `git2::Repository`.
  - Optionally constrains output to a single workspace package via `--package` by using `gen_changelog::RustPackages`.
  - Builds a `ChangeLog` through `ChangeLog::builder()`:
    - config (`ChangeLogConfig`)
    - summary flag
    - optional rust package filter
    - `walk_repository()` to populate sections/links
    - optional `update_unreleased_to_next_version()`
    - `save()` and/or print.
- `config_cli.rs`:
  - Prints and/or writes a default `gen-changelog.toml` config (via `ChangeLogConfig::save`).

### Library flow (`crates/gen-changelog/src/lib/*`)
Public API is re-exported from `lib.rs`:
- `ChangeLog` / `ChangeLogBuilder` (`change_log.rs`)
- `ChangeLogConfig` (`change_log_config.rs`)
- `RustPackages` for `--package` filtering (`package.rs`)
- `Error` (`error.rs`)

Key internal modules:
- `change_log.rs` (core orchestration):
  - Extracts GitHub `owner/repo` from `remote.origin.url`.
  - Collects and sorts version tags (semver) to determine section boundaries.
  - Creates `Section`s and drives commit walking over the correct ranges.
  - Emits reference links (`[Unreleased]: …`, `[1.2.3]: …`) for GitHub compare/release URLs.
- `change_log/section.rs` (commit collection + grouping):
  - Walks commits via `git2::Revwalk` for a given range (`WalkSetup`).
  - Optionally filters commits to a workspace package:
    - include commits that touch files under the package root, OR
    - include commits that look like dependency updates for crates the package depends on.
  - Classifies commits into groups using conventional-commit parsing + `ChangeLogConfig`’s type→group mapping.
- `change_log/section/cc_commit.rs` (conventional commit parsing):
  - Regex-based parsing of the *summary line* (emoji, type, scope, breaking flag, description).
- `change_log_config.rs` (config model + defaults):
  - Default groups/types + publish flags live here.
  - Reads config from `gen-changelog.toml` (or defaults if absent).
  - Group/heading management helpers live under `change_log_config/`.
- `package.rs` (workspace package metadata):
  - Loads the workspace `Cargo.toml` and each member `Cargo.toml` using `cargo_toml::Manifest`.
  - Builds `RustPackages { packages_by_name }` used by the CLI.

## CI and release automation notes
- CI is configured via CircleCI in `.circleci/config.yml` (the repo has no GitHub Actions workflows).
- Releases appear to be driven by **cargo-release** configuration:
  - Root `release.toml` + `release-hook.sh`:
    - rebuilds root `README.md`
    - runs `gen-changelog generate --display-summaries --next-version "$SEMVER"`
  - Crate-local `crates/gen-changelog/release.toml` + `crates/gen-changelog/release-hook.sh`:
    - generates `crates/gen-changelog/CHANGELOG.md` using `NEW_VERSION` (or a positional arg)

When working on release behavior, check both the root and crate-local release configs/scripts, since they target different outputs.