// SPDX-License-Identifier: GPL-3.0-or-later
//! `.sch` / `.sym` parser and writer (layer L1). Operates on `&str`, never the
//! filesystem, so it compiles to wasm. Input is hostile: every failure is an error.
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
pub enum FormatError {
    #[error("not supported yet: {0}")]
    Unsupported(String),
}

/// Parse xschem schematic text.
pub fn parse_schematic(_text: &str) -> Result<Schematic, FormatError> {
    Err(FormatError::Unsupported("schematic parsing".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_is_not_implemented_yet_but_never_panics() {
        assert!(parse_schematic("v {xschem version=3.4.0}").is_err());
    }
}
