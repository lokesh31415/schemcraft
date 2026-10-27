// SPDX-License-Identifier: GPL-3.0-or-later
//! Semantic drawing layers ([`LayerRole`]) and 2D graphic primitives ([`Graphics`]).

use serde::{Deserialize, Serialize};

use crate::geometry::{Point, Rotation};
use crate::props::Properties;

/// Semantic drawing layer role in SchemCraft.
///
/// Using semantic roles allows UI themes (dark/light/print) and AI visibility filters
/// to understand *what* a graphic represents, while [`LayerRole::Custom`] preserves
/// arbitrary numbered layers from formats like xschem without loss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LayerRole {
    /// Electrical wire layer (xschem layer 1).
    Wire,
    /// General text and notes layer (xschem layer 3).
    Text,
    /// Component symbol body graphics (xschem layer 4).
    #[default]
    SymbolBody,
    /// Electrical pin terminal markers (xschem layer 5).
    Pin,
    /// Pin number annotations (xschem layer 13).
    PinAnnotation,
    /// Net name annotations on pins (xschem layer 15).
    NetAnnotation,
    /// Simulation back-annotation text (xschem layer 17).
    SimAnnotation,
    /// Any other numbered CAD layer (e.g. xschem layers 0, 2, 6, 7, 8...),
    /// preserved for lossless round-tripping.
    Custom(u32),
}

/// A cosmetic line segment on a [`LayerRole`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub layer: LayerRole,
    pub a: Point,
    pub b: Point,
    pub props: Properties,
}

/// A cosmetic rectangle/box on a [`LayerRole`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub layer: LayerRole,
    pub a: Point,
    pub b: Point,
    pub props: Properties,
}

/// A circular arc on a [`LayerRole`]. Angles are in degrees (`360.0` = full circle).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arc {
    pub layer: LayerRole,
    pub center: Point,
    pub radius: f64,
    pub start_angle: f64,
    pub arc_angle: f64,
    pub props: Properties,
}

/// A polygon or polyline on a [`LayerRole`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    pub layer: LayerRole,
    pub points: Vec<Point>,
    pub props: Properties,
}

/// A text label or attribute template (`@name`, `@value`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    pub pos: Point,
    pub rot: Rotation,
    pub flip: bool,
    pub xscale: f64,
    pub yscale: f64,
    pub layer: LayerRole,
    pub props: Properties,
}

/// 2D graphic primitives shared by both [`crate::Symbol`]s and [`crate::Schematic`]s.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Graphics {
    pub lines: Vec<Line>,
    pub rects: Vec<Rect>,
    pub arcs: Vec<Arc>,
    pub polygons: Vec<Polygon>,
    pub texts: Vec<Text>,
}

impl Graphics {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
            && self.rects.is_empty()
            && self.arcs.is_empty()
            && self.polygons.is_empty()
            && self.texts.is_empty()
    }
}
