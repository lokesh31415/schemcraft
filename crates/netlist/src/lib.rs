// SPDX-License-Identifier: GPL-3.0-or-later
//! Netlisting (layer L1). Pure functions from a schematic to text.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

use schemcraft_core::Schematic;

#[derive(Debug, thiserror::Error)]
pub enum NetlistError {
    #[error("not supported yet: {0}")]
    Unsupported(String),
}

/// Generate a SPICE netlist.
pub fn spice(_schematic: &Schematic) -> Result<String, NetlistError> {
    Err(NetlistError::Unsupported("SPICE netlisting".into()))
}
