# SchemCraft

An open-source, AI-first schematic capture and netlisting tool written in Rust, compatible with
xschem `.sch`/`.sym` files. Modern egui/wgpu UI, cross-platform desktop and web, and a command
model that humans and AI agents share.

> Status: early scaffold. Not usable yet. See `plan/STATUS.md`.

SchemCraft is an independent project; it is not affiliated with or endorsed by xschem or its author.

License: GPL-3.0-or-later (see `LICENSE`, `ATTRIBUTION.md`).

## Develop
```sh
cargo test --workspace
cargo xtask layers
```
