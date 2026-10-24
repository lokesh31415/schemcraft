# SchemCraft

An open-source, AI-first schematic capture and netlisting tool written in Rust, compatible with
xschem `.sch` / `.sym` files. Modern `egui` + `wgpu` UI, cross-platform desktop and web, and a
unified command model that humans and AI agents share.

> [!NOTE]
> **Early Work-in-Progress & Learning-First Repository**
>
> SchemCraft is being built from scratch in the open—step by step—as both a serious modern EDA tool
> and a **living educational handbook** for developers learning **Rust** and **Schematic Capture /
> EDA internals** at the same time.
>
> - **Current stage:** Milestone M0 (Workspace scaffold & architecture foundation). It is **not yet
>   usable as an end-user application**. Follow progress in [`plan/STATUS.md`](./plan/STATUS.md) and
>   [`plan/execution-plan.md`](./plan/execution-plan.md).
> - **Want to learn how schematic capture or Rust works?** Start in [`docs/README.md`](./docs/README.md).
>   Every feature lands in small, self-contained teaching phases alongside clear explanations of both
>   the EDA concepts and the Rust patterns used.

---

## Disclaimer & Attribution

SchemCraft is an independent open-source project; it is **not affiliated with or endorsed by
xschem or its author (Stefan Frederik Schippers)**. Please do not report SchemCraft issues to the
upstream xschem project.

- **License:** `GPL-3.0-or-later` (see [`LICENSE`](./LICENSE) and [`ATTRIBUTION.md`](./ATTRIBUTION.md)).

## Development Quickstart

```sh
cargo test --workspace
cargo xtask layers
```
