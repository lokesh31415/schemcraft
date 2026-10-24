// SPDX-License-Identifier: GPL-3.0-or-later
//! SchemCraft core data model (layer L0). No UI, I/O or platform dependencies,
//! so it compiles to wasm.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

use serde::{Deserialize, Serialize};

/// A point in schematic coordinates (xschem uses f64 user units).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// A wire segment connecting two points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wire {
    pub a: Point,
    pub b: Point,
    /// Net label property string (xschem `lab=`), if any.
    pub label: Option<String>,
}

/// A schematic: the objects that make up one `.sch` document.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Schematic {
    pub wires: Vec<Wire>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_schematic_has_no_wires() {
        assert!(Schematic::default().wires.is_empty());
    }
}
