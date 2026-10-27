# 3. Algebraic Data Types (`enum`), Composition, and `BTreeMap` in `schemcraft-core`

In **M1 — Teaching Phase 1** (and [ADR 0002](../../plan/adr/0002-canonical-core-and-format-codecs.md)), we designed SchemCraft's canonical domain model across four focused modules in [`crates/core/src/`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/lib.rs):
- [`geometry.rs`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/geometry.rs) (`Point`, `Rotation`)
- [`props.rs`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/props.rs) (`Properties`, `XschemHeaders`)
- [`graphics.rs`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/graphics.rs) (`LayerRole`, `Graphics`, `Line`, `Rect`, `Arc`, `Polygon`, `Text`)
- [`document.rs`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/document.rs) (`Pin`, `PinDirection`, `Wire`, `Instance`, `Symbol`, `Schematic`, `Document`)

Here are the four core Rust concepts used in those files and why they make our architecture so clean.

---

## 1. Rust's Superpower: Algebraic Data Types (`enum` Variants That Hold Data)

In C, Java, or Python, an `enum` is usually just a list of named integers (`0, 1, 2, 3`).
Rust supports C-style enums too—like our [`Rotation`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/geometry.rs#L23-L30):

```rust
pub enum Rotation {
    #[default]
    Deg0 = 0,
    Deg90 = 1,
    Deg180 = 2,
    Deg270 = 3,
}
```

**But in Rust, each variant of an `enum` can also carry its own data of any type!** Computer scientists call this an **Algebraic Data Type (ADT)** or **Tagged Union**. We use this in three places in `schemcraft-core`:

### A. [`LayerRole`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/graphics.rs#L14-L34) — Semantic Cases + a Parameterized Escape Hatch
```rust
pub enum LayerRole {
    Wire,
    Text,
    #[default]
    SymbolBody,
    Pin,
    PinAnnotation,
    NetAnnotation,
    SimAnnotation,
    Custom(u32), // <-- This variant carries a u32 layer number inside it!
}
```
Notice how the first 7 variants carry no extra data, while `LayerRole::Custom(u32)` carries a `u32` number (like `LayerRole::Custom(8)`). You can only access that `u32` by pattern-matching on `LayerRole::Custom(num)`.

### B. [`PinDirection`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/document.rs#L12-L20) — Known Directions + Custom String Fallback
```rust
pub enum PinDirection {
    In,
    Out,
    #[default]
    InOut,
    Custom(String), // <-- Carries an owned String if a file had a custom direction!
}
```

### C. [`Document`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/document.rs#L152-L156) — Wrapping Two Different Structs Under One Type
```rust
pub enum Document {
    Schematic(Schematic),
    Symbol(Symbol),
}
```
Because `Schematic` and `Symbol` are two distinct structs, a function that can load *either* a schematic or a symbol returns `Result<Document, FormatError>`. Callers then use `match` (or our helper methods [`as_schematic()`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/document.rs#L160-L165) and [`as_symbol()`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/document.rs#L168-L173)) to handle each case safely.

---

## 2. Struct Composition Over Duplication (`Graphics` and `Symbol`)

Both a [`Symbol`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/document.rs#L125-L135) and a [`Schematic`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/document.rs#L138-L149) can contain 2D drawings (`lines`, `rects`, `arcs`, `polygons`, `texts`).

Instead of copy-pasting those 5 `Vec` fields into both `Symbol` and `Schematic` (or using object-oriented inheritance, which Rust intentionally does not have), we group them into [`Graphics`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/graphics.rs#L88-L95):

```rust
pub struct Graphics {
    pub lines: Vec<Line>,
    pub rects: Vec<Rect>,
    pub arcs: Vec<Arc>,
    pub polygons: Vec<Polygon>,
    pub texts: Vec<Text>,
}
```
And both `Symbol` and `Schematic` embed `pub graphics: Graphics`!
- Any rendering function that draws 2D shapes only needs to accept `&Graphics`—it automatically works for both symbols and schematics!
- Furthermore, because `Instance` (which lives inside `Schematic`) embeds `pub embedded_symbol: Option<Symbol>`, `Schematic` is no longer a self-referential recursive type—meaning we don't even need `Box<...>` heap indirection!

---

## 3. `BTreeMap` vs. `HashMap` — Deterministic Key Order

Every object has a [`Properties`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/props.rs#L13-L16) struct:

```rust
use std::collections::BTreeMap;

pub struct Properties {
    pub map: BTreeMap<String, String>,
    pub raw: Option<String>,
}
```

Why did we choose `std::collections::BTreeMap` instead of `std::collections::HashMap`?
- **`HashMap`** uses a random per-process seed, so iterating over keys produces a **different random order every time you run the program**.
- In an EDA tool, non-deterministic output causes noisy `git diff`s when saving files and flaky netlist tests.
- **`BTreeMap`** keeps keys sorted alphabetically at all times, making file writers, netlists, and JSON outputs **100% deterministic**.

---

## 4. Ergonomic String Arguments with `impl Into<String>`

Look at [`Pin::new`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/document.rs#L63-L71) and [`Properties::set`](file:///Users/loki/Workspace/GitHub_Repos/xschem_rust/crates/core/src/props.rs#L27-L30):

```rust
pub fn new(name: impl Into<String>, dir: PinDirection, pos: Point) -> Self {
    Self {
        name: name.into(),
        dir,
        pos,
        size: DEFAULT_PIN_SIZE,
        props: Properties::default(),
    }
}
```
By taking `impl Into<String>` instead of `String`, callers can write `Pin::new("P", ...)` with a string literal `&str` **or** pass an owned `String` directly!
