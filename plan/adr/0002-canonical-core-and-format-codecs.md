# ADR 0002 — Canonical Core Domain Model and Pluggable Format Codecs

## Context
Initially during M1, `crates/core` modeled xschem's `.sch`/`.sym` file records almost 1-to-1
(e.g. representing electrical pins as rectangles on magic layer number `5`, using raw integer
layer numbers, and using a single `Schematic` struct for both schematics and symbols).

While compatible with xschem, tightly coupling `L0 schemcraft-core` to xschem's 1990s file
format quirks would force `netlist`, `engine`, `ui-egui`, and AI commands to deal with format
quirks and would make supporting SchemCraft's own native format or other industry formats
(KiCad, LTspice, EDIF) difficult.

## Decision
1. **Canonical Domain Model in `L0 schemcraft-core`:**
   - **Semantic Layers (`LayerRole`):** `core` uses semantic layer roles (`Wire`, `Text`,
     `SymbolBody`, `Pin`, `PinAnnotation`, `NetAnnotation`, `SimAnnotation`, plus `Custom(u32)`
     to preserve arbitrary numbered layers losslessly).
   - **First-Class `Pin` and `PinDirection`:** Electrical pins on a symbol are represented by a
     dedicated `Pin` struct (`name`, `dir`, `pos`, `size`, `props`), not as magic-layer boxes.
   - **Distinct `Schematic` and `Symbol` Structs + `enum Document`:**
     - `Symbol` holds `pins: Vec<Pin>`, `graphics: Graphics`, `props: Properties`.
     - `Schematic` holds `wires: Vec<Wire>`, `instances: Vec<Instance>`, `graphics: Graphics`,
       `props: Properties`.
     - Both share a common `Graphics` struct (`lines`, `rects`, `arcs`, `polygons`, `texts`) and
       an optional `xschem_headers: Option<XschemHeaders>` passthrough for lossless `.sch`/`.sym`
       round-tripping.
     - `enum Document { Schematic(Schematic), Symbol(Symbol) }` wraps either document kind.
2. **Pluggable Format Codecs in `L1 schemcraft-format`:**
   - `format::xschem`: Parses and writes xschem `.sch` and `.sym` files via an internal
     `XschemAst` and translates to/from `schemcraft_core::Document` (converting layer-5 boxes
     into `core::Pin`s and back).
   - `format::native`: SchemCraft's own native format (`.scsch` / `.scsym`) using **KiCad-style
     S-expressions** (one line per wire/pin/shape for clean `git diff`s, named fields for
     forward compatibility, zero external dependencies).
   - JSON serialization remains available out of the box via `serde` on all `core` types for
     MCP, CLI, and web interchange.
   - Future industry format parsers (KiCad, LTspice, EDIF) will live as peer codec modules in
     `crates/format` translating into `schemcraft_core::Document`.
