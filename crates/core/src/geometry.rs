// SPDX-License-Identifier: GPL-3.0-or-later
//! 2D geometry primitives (`Point`, `Rotation`) in schematic user coordinates.

use serde::{Deserialize, Serialize};

/// A 2D point or size vector in schematic user coordinates (`f64`).
///
/// Convention: `+x` points right and `+y` points **down** (matching xschem and KiCad
/// schematic canvas coordinates).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Quarter-turn rotation (`0`, `1`, `2`, `3`, corresponding to 0°, 90°, 180°, 270°).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Rotation {
    #[default]
    Deg0 = 0,
    Deg90 = 1,
    Deg180 = 2,
    Deg270 = 3,
}

impl Rotation {
    /// Convert a quarter-turn integer (`0..=3`) into a [`Rotation`].
    #[must_use]
    pub const fn from_quarter_turns(turns: u16) -> Option<Self> {
        match turns {
            0 => Some(Self::Deg0),
            1 => Some(Self::Deg90),
            2 => Some(Self::Deg180),
            3 => Some(Self::Deg270),
            _ => None,
        }
    }

    /// Return the integer quarter-turn code (`0..=3`).
    #[must_use]
    pub const fn quarter_turns(self) -> u16 {
        match self {
            Self::Deg0 => 0,
            Self::Deg90 => 1,
            Self::Deg180 => 2,
            Self::Deg270 => 3,
        }
    }

    /// Return the angle in degrees (`0`, `90`, `180`, or `270`).
    #[must_use]
    pub const fn degrees(self) -> u16 {
        self.quarter_turns() * 90
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_converts_valid_quarter_turns_and_rejects_invalid() {
        assert_eq!(Rotation::from_quarter_turns(0), Some(Rotation::Deg0));
        assert_eq!(Rotation::from_quarter_turns(1), Some(Rotation::Deg90));
        assert_eq!(Rotation::from_quarter_turns(2), Some(Rotation::Deg180));
        assert_eq!(Rotation::from_quarter_turns(3), Some(Rotation::Deg270));
        assert_eq!(Rotation::from_quarter_turns(4), None);
        assert_eq!(Rotation::Deg270.degrees(), 270);
    }
}
