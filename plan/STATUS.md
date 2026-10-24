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
- **EDA (`docs/01-eda-and-xschem/01-what-is-schematic-capture.md`):** Cosmetic graphics vs. electrical objects, symbols (`.sym`) vs. instances (`.sch`), pins, wires vs. nets, hierarchy, netlisting overview.
- **Rust (`docs/02-rust-for-schemcraft/`):** Cargo workspaces, library vs. binary crates, strict `L0`–`L3` layering (`01-workspace-and-crates.md`), `struct`, `#[derive]`, `Option<T>`, `Result<T, E>`, `?`, and banning `unwrap()` (`02-structs-enums-and-errors.md`).
- **Architecture (`docs/03-architecture/01-command-engine-and-ai.md`):** Command-first engine (`Session::execute`), borrowing (`&mut self`, `&str`, `&Value`).

## Active Focus
- **Current Milestone:** **M1 — `.sch` / `.sym` File Format & Core Data Model** (`crates/core`, `crates/format`).
- **Completed Milestones:** M0 — Workspace scaffold, `xtask layers` check, `AGENTS.md` rules, and `docs/` learning handbook foundation.

## Last Completed Slice
- Set up the Cargo workspace (`core`, `format`, `netlist`, `engine`, `ui-egui`, `schemcraft-cli`, `xtask`), GPL-3.0-or-later licensing, `ATTRIBUTION.md`, and the initial `docs/` handbook.
- Added strict Teaching-Phase rules, seamless session-continuity rules in `plan/STATUS.md`, and the **No Auto-Commit** rule in `AGENTS.md`.

## Immediate Next Slice (Start M1 Here)
- **M1 — Teaching Phase 1 (Inspect & Understand the xschem File Format):**
  1. Read a real symbol file (e.g. `reference/xschem/xschem_library/devices/res.sym`) and schematic file from `reference/xschem/xschem_library`, plus `reference/xschem/src/save.c`.
  2. Explain how xschem's ASCII record format works (`v`, `G`, `K`, `V`, `S`, `E`, `L`, `B`, `A`, `P`, `T`, `N`, `C`, and `{key=value}` property blocks) and write `docs/01-eda-and-xschem/02-xschem-file-format.md`.
  3. Design the core Rust types in `crates/core/src/lib.rs` (teaching Rust `enum` algebraic data types, `HashMap`/`BTreeMap` for properties, and coordinate types) before writing the parser in Phase 2.

## Key Context & Rules to Remember
- **No Auto-Commit:** Never run `git commit` or `git push` without explicit user approval in the current turn.
- **Reference repos (`reference/xschem` and `reference/solidcraft`):** Gitignored locally for study/porting; record any ported logic in `ATTRIBUTION.md`.
- **Checks:** `cargo fmt --check && cargo clippy --workspace --all-targets && cargo test --workspace && cargo xtask layers`.
