// SPDX-License-Identifier: GPL-3.0-or-later
//! SchemCraft canonical domain model (layer L0).
//!
//! No UI, I/O or platform dependencies, so it compiles cleanly to `wasm32-unknown-unknown`.
//!
//! ### Module Layout
//! - [`geometry`] — 2D coordinate primitives ([`Point`], [`Rotation`]).
//! - [`props`] — Deterministic key-value [`Properties`] and [`XschemHeaders`] passthrough.
//! - [`graphics`] — Semantic [`LayerRole`] and 2D drawing primitives ([`Graphics`], [`Line`],
//!   [`Rect`], [`Arc`], [`Polygon`], [`Text`]).
//! - [`document`] — Electrical domain objects ([`Pin`], [`PinDirection`], [`Wire`], [`Instance`])
//!   and top-level [`Symbol`], [`Schematic`], and [`Document`] types.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

pub mod document;
pub mod geometry;
pub mod graphics;
pub mod props;

pub use document::{
    DEFAULT_PIN_SIZE, Document, Instance, Pin, PinDirection, Schematic, Symbol, Wire,
};
pub use geometry::{Point, Rotation};
pub use graphics::{Arc, Graphics, LayerRole, Line, Polygon, Rect, Text};
pub use props::{Properties, XschemHeaders};
