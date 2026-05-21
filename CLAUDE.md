# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`ferro_find` is an indexer / file-finder app written in Rust (edition 2024) — think macOS Spotlight, but cross-platform and built in Rust. The intended shape is a background indexer + live filesystem watcher feeding a persistent store that a fast search front-end queries.

It's in an early-bootstrap state: only the `scan` subcommand and a directory walker exist; most modules listed below are empty placeholders awaiting implementation.

## Commands

- Build (debug): `cargo build`
- Build (release): `cargo build --release`
- Run: `cargo run -- scan <path>` (e.g., `cargo run -- scan .`)
- Test: `cargo test` — run a single test with `cargo test <test_name>`
- Lint: `cargo clippy --all-targets`
- Format: `cargo fmt`

## Architecture

The project is laid out as a single binary crate with module folders pre-scaffolded for the intended design, even though most are still empty `mod.rs` files. Treat the directory structure as the planned architecture:

- `src/main.rs` — clap entry point. Dispatches subcommands (currently only `Scan { path }`).
- `src/scanner/` — directory traversal. `scanner::scan_directory` uses `walkdir::WalkDir`, skips non-files and metadata errors silently, and returns `Vec<FileEntry>`.
- `src/models/` — shared data types. `FileEntry { path, filename, size }` is the unit returned by the scanner and is intended to flow into indexer/search/db layers.
- `src/indexer/`, `src/search/`, `src/db/` — planned pipeline: indexer consumes scanner output, builds searchable structures, persists via db.
- `src/watcher/` — planned filesystem-change watching to keep the index live.
- `src/configs/` — planned settings + ignore-rule handling (`settings.rs`, `ignore_rules.rs` stubs exist).
- `src/platform/` — planned OS-specific shims (Windows is the primary dev target — see edition 2024 + Windows paths).
- `src/services/`, `src/ui/`, `src/utils/`, `src/errors/` — placeholders for cross-cutting concerns.

When adding new modules, follow the existing pattern: a folder with `mod.rs` re-exporting submodules (see `src/scanner/mod.rs` and `src/models/mod.rs`), and register the top-level module in `src/main.rs`.

## Conventions

- Rust **edition 2024** — keep that in mind for syntax (e.g., `let-else`, edition-gated lints).
- The scanner currently swallows errors silently (`Err(_) => continue`). When wiring in `src/errors/`, replace these with proper error propagation rather than preserving the silent-skip behavior.
