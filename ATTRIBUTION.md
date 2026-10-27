# Attribution

SchemCraft is licensed GPL-3.0-or-later. This file records code, formats and ideas taken from
other projects. Add a row whenever ported or copied material lands in the repo.

| Area in SchemCraft | Source project | Source files | License | Notes |
|---|---|---|---|---|
| `.sch`/`.sym` data model (`crates/core`) & format spec (`docs/01-eda-and-xschem/02-xschem-file-format.md`) | xschem (Stefan Frederik Schippers) | `src/xschem.h`, `src/save.c`, `xschem_library/devices/res.sym` | GPL-2.0-or-later | Clean-room Rust data structures modeling xschem's ASCII record format and layer constants (`LAYER_WIRE`, `LAYER_PIN`, etc.). |

## Inspiration & Planned Sources
- **xschem** ([Stefan Frederik Schippers](https://github.com/StefanSchippers/xschem), `GPL-2.0-or-later`):
  SchemCraft is inspired by xschem and aims for compatibility with its `.sch`/`.sym` file formats
  (`save.c`), netlisting (`netlist.c`, `spice_netlist.c`), and label/bus expansion (`expandlabel.y`).
- **SolidCraft** (`MIT OR Apache-2.0`): workspace conventions, command-engine architecture, and
  `xtask` patterns.

SchemCraft is an independent project and is not affiliated with or endorsed by xschem or its author.
