// SPDX-License-Identifier: GPL-3.0-or-later
//! Engine (layer L2): the single command surface used by the UI, CLI, MCP
//! server, scripts and the built-in assistant.
//!
//! Command ids are our own dotted lower-case names: `<area>.<thing>[.<variant>]`.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

use schemcraft_core::Schematic;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("invalid parameters: {0}")]
    InvalidParams(String),
}

/// An open design plus its command history.
#[derive(Debug, Default)]
pub struct Session {
    pub schematic: Schematic,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    /// Run a command by id with JSON parameters. Never opens a dialog.
    pub fn execute(&mut self, id: &str, _params: &Value) -> Result<Value, EngineError> {
        match id {
            "sch.wire.count" => Ok(Value::from(self.schematic.wires.len())),
            other => Err(EngineError::UnknownCommand(other.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_wires_through_a_command() {
        let mut s = Session::new();
        let r = s.execute("sch.wire.count", &Value::Null);
        assert_eq!(r.ok(), Some(Value::from(0)));
    }

    #[test]
    fn unknown_command_is_an_error_not_a_panic() {
        let mut s = Session::new();
        assert!(s.execute("nope", &Value::Null).is_err());
    }
}
