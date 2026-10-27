# Welcome to the SchemCraft Handbook

This `docs/` folder is written for **humans learning Rust, Electronic Design Automation (EDA), and SchemCraft at the same time**. You do not need prior experience with Rust or schematic capture internals to feel at home here.

Every time we build a feature in SchemCraft, we update this handbook using **real code from our repository** so nothing in the codebase ever feels like a black box.

---

## Reading Order

### Part 1: EDA & Xschem Concepts (`01-eda-and-xschem/`)
Learn how schematic capture tools actually work under the hood.
1. [What is Schematic Capture?](./01-eda-and-xschem/01-what-is-schematic-capture.md) — Drawings vs. circuits, symbols vs. instances, wires vs. nets, and what a netlister does.
2. [The Xschem `.sch` & `.sym` File Format](./01-eda-and-xschem/02-xschem-file-format.md) — How xschem stores schematics and symbols as plain ASCII records (`v`, `K`, `L`, `B`, `A`, `P`, `T`, `N`, `C`).
3. *(Coming in M3)* `03-connectivity-and-netlisting.md` — How graphic coordinates turn into a SPICE circuit simulation file.

### Part 2: Rust for SchemCraft (`02-rust-for-schemcraft/`)
Learn Rust step-by-step through the exact patterns used in this repo.
1. [Workspaces, Crates, and Layering](./02-rust-for-schemcraft/01-workspace-and-crates.md) — How a multi-crate Rust project is organized and why we enforce strict layers (`L0` to `L3`).
2. [Structs, Enums, and Error Handling](./02-rust-for-schemcraft/02-structs-enums-and-errors.md) — How our scaffold models data (`struct`, `enum`, `Option`, `Result`) and why we ban `unwrap()`.
3. [Enums, `BTreeMap`, and Iterators in `schemcraft-core`](./02-rust-for-schemcraft/03-enums-and-collections.md) — How we model rotations, deterministic property maps, and zero-allocation pin iterators.

### Part 3: SchemCraft Architecture (`03-architecture/`)
Learn how SchemCraft's engine, UI, and AI assistant fit together.
1. [The Command Engine & AI-First Design](./03-architecture/01-command-engine-and-ai.md) — Why "everything is a command" lets the UI, CLI, and AI agents share the exact same brain.
