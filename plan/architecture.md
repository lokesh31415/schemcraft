# Architecture

```
L0 core      canonical domain model (Document, Schematic, Symbol, Pin, LayerRole, Graphics)
L1 format    pluggable codecs: format::xschem (.sch/.sym), format::native (.scsch/.scsym S-expr)
L1 netlist   connectivity, SPICE/Verilog/VHDL netlisters operating on L0 core
L2 engine    command registry, session, undo, AI-edit grouping + review queue
L3 ui-egui   egui/eframe/wgpu shell, canvas, assistant panel
apps         schemcraft (desktop), schemcraft-cli (headless + MCP), schemcraft-web (wasm)
```

Planned later crates: `render` (scene → GPU, hit-test, spatial index), `mcp`, `assistant`
(agent/endpoint connectors, UI-independent), `sim` (ngspice).

## Canonical Model + Pluggable Format Codecs ([ADR 0002](./adr/0002-canonical-core-and-format-codecs.md))
- `L0 core` defines SchemCraft's format-agnostic domain model: semantic layers (`LayerRole`),
  first-class `Pin`s, and distinct `Symbol` and `Schematic` structs wrapped in `Document`.
- `L1 format` houses modular format codecs (`xschem` for `.sch`/`.sym`, `native` for KiCad-style
  `.scsch`/`.scsym` S-expressions, and future industry formats) that translate into `L0 core`.

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
