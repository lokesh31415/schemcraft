// SPDX-License-Identifier: GPL-3.0-or-later
//! Deterministic key-value [`Properties`] and format header metadata.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A deterministic key-value property map attached to a schematic, symbol, or object.
///
/// Uses [`BTreeMap`] so iteration order, netlist generation, and serialized files
/// are 100% deterministic across runs. Also stores an optional `raw` string when
/// loaded from a format like xschem so untouched multi-line blocks round-trip verbatim.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Properties {
    pub map: BTreeMap<String, String>,
    pub raw: Option<String>,
}

impl Properties {
    /// Look up a property value by key (e.g. `props.get("name")` or `props.get("value")`).
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(String::as_str)
    }

    /// Insert or update a key-value pair, clearing any cached `raw` string so writers
    /// re-serialize `self.map`.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.map.insert(key.into(), value.into());
        self.raw = None;
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty() && self.raw.as_deref().unwrap_or("").is_empty()
    }
}

/// Optional passthrough of xschem-specific global header blocks (`v`, `G`, `K`, `V`, `S`, `F`, `E`)
/// so files imported from xschem can round-trip back to `.sch`/`.sym` without losing metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct XschemHeaders {
    /// `v {...}` version and file comment block.
    pub version: String,
    /// `G {...}` global VHDL properties.
    pub vhdl: Properties,
    /// `K {...}` global symbol attributes (`type`, `format`, `template`, etc.).
    pub symbol: Properties,
    /// `V {...}` global Verilog properties.
    pub verilog: Properties,
    /// `S {...}` global SPICE properties.
    pub spice: Properties,
    /// `F {...}` global Spectre properties (optional in older xschem files).
    pub spectre: Option<Properties>,
    /// `E {...}` global tEDAx properties.
    pub tedax: Properties,
}
