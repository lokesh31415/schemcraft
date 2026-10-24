# Execution plan

| Milestone | Goal | Proof |
|---|---|---|
| M0 | Workspace scaffold, AGENTS.md, layer check, CI | tests + `cargo xtask layers` green |
| M1 | `.sch`/`.sym` parser and writer | xschem_library files round-trip |
| M2 | Read-only egui/wgpu viewer: pan, zoom, hierarchy | opens real designs on desktop |
| M3 | Connectivity + SPICE netlister | output matches xschem on sample designs |
| M4 | Editing commands, undo/redo, selection | every edit is a command with tests |
| M5 | CLI + MCP server + control channel | agent can open, edit, netlist, verify |
| M6 | Built-in assistant panel; review-before-apply | edits grouped, reviewable, undoable |
| M7 | Web build (wasm) | viewer + editor in the browser |
| M8 | ngspice integration, waveform viewer | edit → simulate → inspect loop |
| M9 | Scripting layer (Tcl compatibility decision) | `xschemrc` / `tcleval` strategy ADR |

## Autonomous operation
Pick the next task from `STATUS.md`, do it, run the checks in `AGENTS.md`, update `STATUS.md`.
Ask the owner only for: license/name changes, new heavy dependencies, scripting-language choice,
anything touching user data safety.
