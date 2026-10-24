# ADR 0001 — License, name, UI stack

- **Name:** SchemCraft (crates `schemcraft-*`). Independent of xschem.
- **License:** GPL-3.0-or-later. xschem sources are GPL-2.0-or-later, so porting is permitted;
  Apache/MIT dependencies are compatible with GPL-3.0. Two xschem files are GPL-3 (compatible).
- **Not clean-room** with respect to xschem; attribution recorded in `ATTRIBUTION.md`.
  Commercial EDA products are off limits as code/asset/text sources.
- **UI:** egui + eframe + wgpu, isolated in `ui-egui`; swappable.
- **Assistant:** bring-your-own agent (CLI agents via ACP/MCP, OpenAI/Anthropic-compatible endpoints);
  no bundled model; global off switch; optional review-before-apply.
- **Web:** first-class target via wasm-clean core crates.
