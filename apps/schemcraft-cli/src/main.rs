// SPDX-License-Identifier: GPL-3.0-or-later
use schemcraft_engine::Session;
use serde_json::Value;

fn main() {
    let mut session = Session::new();
    match session.execute("sch.wire.count", &Value::Null) {
        Ok(v) => println!("wires: {v}"),
        Err(e) => eprintln!("error: {e}"),
    }
}
