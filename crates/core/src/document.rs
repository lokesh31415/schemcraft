// SPDX-License-Identifier: GPL-3.0-or-later
//! Electrical objects ([`Pin`], [`Wire`], [`Instance`]) and top-level documents
//! ([`Symbol`], [`Schematic`], [`Document`]).

use serde::{Deserialize, Serialize};

use crate::geometry::{Point, Rotation};
use crate::graphics::Graphics;
use crate::props::{Properties, XschemHeaders};

/// Electrical direction of a [`Pin`] (`in`, `out`, `inout`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PinDirection {
    In,
    Out,
    #[default]
    InOut,
    /// Preserves any non-standard `dir=` string from imported files.
    Custom(String),
}

impl PinDirection {
    #[must_use]
    pub fn from_str_lossless(s: &str) -> Self {
        match s {
            "in" => Self::In,
            "out" => Self::Out,
            "inout" | "" => Self::InOut,
            other => Self::Custom(other.to_owned()),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::In => "in",
            Self::Out => "out",
            Self::InOut => "inout",
            Self::Custom(s) => s.as_str(),
        }
    }
}

/// Default width/height (`5.0` × `5.0`) of a pin terminal marker box.
pub const DEFAULT_PIN_SIZE: Point = Point::new(5.0, 5.0);

/// A first-class electrical pin on a [`Symbol`].
///
/// In SchemCraft's canonical model, a pin is a dedicated domain object with a
/// connection point (`pos`), direction (`dir`), and name—not a rectangle hidden
/// on a magic layer number.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pin {
    /// Pin identifier (e.g. `"P"`, `"M"`, `"IN"`, `"OUT"`, `"VDD"`).
    pub name: String,
    /// Electrical direction (`In`, `Out`, `InOut`).
    pub dir: PinDirection,
    /// Connection point in symbol coordinates where wires attach.
    pub pos: Point,
    /// Visual marker box size (`width`, `height`), defaulting to `5.0 × 5.0`.
    pub size: Point,
    /// Additional attributes (`pinnumber`, `propag`, `goto`, etc.).
    pub props: Properties,
}

impl Pin {
    /// Create a new pin at `pos` with [`DEFAULT_PIN_SIZE`] (`5.0 × 5.0`).
    #[must_use]
    pub fn new(name: impl Into<String>, dir: PinDirection, pos: Point) -> Self {
        Self {
            name: name.into(),
            dir,
            pos,
            size: DEFAULT_PIN_SIZE,
            props: Properties::default(),
        }
    }

    /// Bounding box corners `(min_pt, max_pt)` of the pin marker centered at `pos`.
    #[must_use]
    pub fn bounds(&self) -> (Point, Point) {
        let half_w = self.size.x * 0.5;
        let half_h = self.size.y * 0.5;
        (
            Point::new(self.pos.x - half_w, self.pos.y - half_h),
            Point::new(self.pos.x + half_w, self.pos.y + half_h),
        )
    }
}

/// An electrical wire segment connecting two points in a [`Schematic`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wire {
    pub a: Point,
    pub b: Point,
    pub props: Properties,
}

impl Wire {
    /// Net label attached to this wire (`lab=`), if any.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.props.get("lab")
    }
}

/// A placed component instance inside a [`Schematic`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    /// Symbol reference string (e.g. `"res.sym"` or `"devices/res.sym"`).
    pub symbol: String,
    /// Instance origin in schematic coordinates.
    pub pos: Point,
    pub rot: Rotation,
    pub flip: bool,
    pub props: Properties,
    /// Optional inline symbol definition (e.g. from xschem `[ ... ]` blocks).
    pub embedded_symbol: Option<Symbol>,
}

impl Instance {
    /// Instance reference designator (`name=`, e.g. `"R1"`, `"C1"`, `"X1"`), if set.
    #[must_use]
    pub fn instance_name(&self) -> Option<&str> {
        self.props.get("name")
    }
}

/// A reusable component symbol blueprint (`.scsym` or `.sym`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    /// Electrical connection pins exposed by this symbol.
    pub pins: Vec<Pin>,
    /// Visual 2D body graphics (`lines`, `rects`, `arcs`, `polygons`, `texts`).
    pub graphics: Graphics,
    /// Symbol-level properties (`type`, `format`, `template`, etc.).
    pub props: Properties,
    /// Optional xschem header passthrough for lossless `.sym` round-tripping.
    pub xschem_headers: Option<XschemHeaders>,
}

/// A circuit schematic design (`.scsch` or `.sch`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Schematic {
    /// Electrical wires connecting pins and net labels.
    pub wires: Vec<Wire>,
    /// Placed component instances.
    pub instances: Vec<Instance>,
    /// Cosmetic schematic annotations (`lines`, `rects`, `arcs`, `polygons`, `texts`).
    pub graphics: Graphics,
    /// Schematic-level properties.
    pub props: Properties,
    /// Optional xschem header passthrough for lossless `.sch` round-tripping.
    pub xschem_headers: Option<XschemHeaders>,
}

/// A top-level SchemCraft document: either a [`Schematic`] or a [`Symbol`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Document {
    Schematic(Schematic),
    Symbol(Symbol),
}

impl Document {
    #[must_use]
    pub const fn as_schematic(&self) -> Option<&Schematic> {
        match self {
            Self::Schematic(sch) => Some(sch),
            Self::Symbol(_) => None,
        }
    }

    #[must_use]
    pub const fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            Self::Symbol(sym) => Some(sym),
            Self::Schematic(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_schematic_has_no_wires() {
        assert!(Schematic::default().wires.is_empty());
    }

    #[test]
    fn first_class_pin_computes_bounds_from_center_and_size() {
        let mut pin = Pin::new("P", PinDirection::InOut, Point::new(0.0, -30.0));
        pin.props.set("pinnumber", "1");

        assert_eq!(pin.name, "P");
        assert_eq!(pin.dir.as_str(), "inout");
        assert_eq!(
            pin.bounds(),
            (Point::new(-2.5, -32.5), Point::new(2.5, -27.5))
        );
    }

    #[test]
    fn symbol_and_schematic_wrap_cleanly_in_document_enum() {
        let mut sym = Symbol::default();
        sym.pins
            .push(Pin::new("A", PinDirection::In, Point::new(-20.0, 0.0)));

        let doc = Document::Symbol(sym);
        assert!(doc.as_symbol().is_some());
        assert!(doc.as_schematic().is_none());
    }

    #[test]
    fn wire_reads_lab_property() {
        let mut props = Properties::default();
        props.set("lab", "VDD");
        let w = Wire {
            a: Point::new(0.0, 0.0),
            b: Point::new(100.0, 0.0),
            props,
        };
        assert_eq!(w.label(), Some("VDD"));
    }
}
