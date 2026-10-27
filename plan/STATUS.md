# Status & Session Context

> Single source of truth for session continuity. Every agent must read this at session start and
> update it at the end of every Teaching Phase.

## Developer Profile & Teaching Mode
- **Rust experience:** Learning Rust alongside building SchemCraft.
- **EDA / Schematic Capture experience:** Learning schematic capture and xschem internals alongside building SchemCraft.
- **Mode:** **Teaching Phases (Strict)** —
  1. **Concept & Mental Model:** Explain the EDA/xschem concept + the Rust pattern first.
  2. **Small, Idiomatic Code Slice:** Write one small, self-contained, testable slice (no black-box dumps).
  3. **Walkthrough & Check-in:** Walk through key lines, update `docs/`, update `plan/STATUS.md`, and pause for review/questions.
- *(Note for agents: If a new contributor starts a session without a profile here, ask their Rust and EDA comfort level first.)*

## Concepts Already Covered in `docs/`
- **EDA (`docs/01-eda-and-xschem/`):**
  - `01-what-is-schematic-capture.md`: Cosmetic graphics vs. electrical objects, symbols (`.sym`) vs. instances (`.sch`), pins, wires vs. nets, hierarchy, netlisting overview.
  - `02-xschem-file-format.md`: Canonical Domain Model vs. Pluggable Codecs (ADR 0002), how xschem's ASCII record format works (`v`, `G`, `K`, `V`, `S`, `F`, `E`, `L`, `B`, `A`, `P`, `T`, `N`, `C`, `[...]`), how `B 5 ...` boxes map to first-class `core::Pin`s, and a preview of SchemCraft's native S-expression format (`.scsch` / `.scsym`).
- **Rust (`docs/02-rust-for-schemcraft/`):**
  - `01-workspace-and-crates.md`: Cargo workspaces, library vs. binary crates, multi-file modules (`pub mod`, `crate::`, and `pub use` Facade re-exports), and strict `L0`–`L3` layering.
  - `02-structs-enums-and-errors.md`: `struct`, `#[derive]`, `Option<T>`, `Result<T, E>`, `?`, and banning `unwrap()`.
  - `03-enums-and-collections.md`: Algebraic Data Types (`enum` variants carrying data: `LayerRole::Custom(u32)`, `PinDirection::Custom(String)`, `Document`), struct composition (`Graphics`), `BTreeMap` vs. `HashMap`, and `impl Into<String>`.
- **Architecture (`docs/03-architecture/01-command-engine-and-ai.md`):** Command-first engine (`Session::execute`), borrowing (`&mut self`, `&str`, `&Value`).

## Active Focus
- **Current Milestone:** **M1 — Canonical Core Model & Pluggable Format Codecs (`xschem` + `native` S-expressions)** (`crates/core`, `crates/format`).
- **Completed Milestones:** M0 — Workspace scaffold, `xtask layers` check, `AGENTS.md` rules, and `docs/` learning handbook foundation.

## Last Completed Slice
- **M1 — Teaching Phase 1 + ADR 0002 (Canonical Core Model & Multi-File Module Refactor):**
  - Recorded `plan/adr/0002-canonical-core-and-format-codecs.md` separating `L0 schemcraft-core` (format-agnostic canonical model) from `L1 schemcraft-format` (pluggable codecs: `format::xschem` and `format::native` S-expressions).
  - Split `crates/core/src/` into four small, focused modules (`geometry.rs`, `props.rs`, `graphics.rs`, `document.rs`) re-exported from a 32-line `crates/core/src/lib.rs`.
  - Updated `docs/` to teach Rust's module system (`pub mod`, `crate::`, `pub use` re-exports) alongside Algebraic Data Types and composition.

## Immediate Next Slice
- **M1 — Teaching Phase 2 (Xschem `{...}` Braced String & `key=value` Property Tokenizer in `crates/format/src/xschem/`):**
  1. Organize `crates/format` into focused submodules (`crates/format/src/xschem/tokens.rs` & `mod.rs`).
  2. Teach character-by-character lexing in Rust (`std::str::Chars` / `Peekable`) and implement the `{...}` braced-string reader/writer and `key=value` `Properties` parser/writer (porting the rules of `load_ascii_string`, `save_ascii_string`, and `get_tok_value` from `reference/xschem/src/save.c` & `token.c`), with unit tests including hostile inputs.
  3. Update `docs/` and `ATTRIBUTION.md`.

## Key Context & Rules to Remember
- **No Auto-Commit:** Never run `git commit` or `git push` without explicit user approval in the current turn.
- **Reference repos (`reference/xschem` and `reference/solidcraft`):** Gitignored locally for study/porting; record any ported logic in `ATTRIBUTION.md`.
- **Checks:** `cargo fmt --check && cargo clippy --workspace --all-targets && cargo test --workspace && cargo xtask layers`.
