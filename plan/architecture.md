# Architecture

```
L0 core      data model, geometry
L1 format    .sch/.sym parse+write      netlist   connectivity, SPICE/Verilog/VHDL
L2 engine    command registry, session, undo, AI-edit grouping + review queue
L3 ui-egui   egui/eframe/wgpu shell, canvas, assistant panel
apps         schemcraft (desktop), schemcraft-cli (headless + MCP), schemcraft-web (wasm)
```

Planned later crates: `render` (scene → GPU, hit-test, spatial index), `mcp`, `assistant`
(agent/endpoint connectors, UI-independent), `sim` (ngspice).

## Agent interface
`Session::execute(id, json) -> json` is the only mutation path. MCP, control channel (localhost,
per-launch token), CLI and assistant are thin adapters over it.

## AI edit safety
- Agent edits arrive as a batch → one undo group.
- Setting `ai.review_before_apply`: batch is staged, shown as diff, applied on approval.
- Read commands: list instances, get net, run ERC, render image.

## Web
Core crates stay wasm-clean. Platform services (file access, ngspice, agent transport) are traits
with desktop and web implementations.
