// SPDX-License-Identifier: GPL-3.0-or-later
//! `cargo xtask layers`: fail if a lower layer depends on a higher one or on UI crates.
use std::{fs, process::ExitCode};

/// (crate dir, layer). A crate may only depend on schemcraft crates of a lower layer.
const LAYERS: &[(&str, u8)] = &[
    ("core", 0),
    ("format", 1),
    ("netlist", 1),
    ("engine", 2),
    ("ui-egui", 3),
];
const UI_DEPS: &[&str] = &["egui", "eframe", "winit", "wgpu", "rfd"];

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("layers") => layers(),
        _ => {
            eprintln!("usage: cargo xtask layers");
            ExitCode::FAILURE
        }
    }
}

fn layers() -> ExitCode {
    let mut bad = false;
    for (dir, layer) in LAYERS {
        let Ok(text) = fs::read_to_string(format!("crates/{dir}/Cargo.toml")) else {
            eprintln!("cannot read crates/{dir}/Cargo.toml");
            bad = true;
            continue;
        };
        for line in text.lines() {
            let name = line.split(['=', '.']).next().unwrap_or("").trim();
            if let Some(dep) = name.strip_prefix("schemcraft-") {
                let dep_layer = LAYERS.iter().find(|(d, _)| *d == dep).map(|(_, l)| *l);
                if dep_layer.is_some_and(|l| l >= *layer) {
                    eprintln!("layer violation: {dir} (L{layer}) depends on {dep}");
                    bad = true;
                }
            }
            if *layer < 3 && UI_DEPS.contains(&name) {
                eprintln!("layer violation: {dir} depends on UI crate {name}");
                bad = true;
            }
        }
    }
    if bad {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
