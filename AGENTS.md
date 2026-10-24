# SchemCraft — instructions for agents

SchemCraft is an open-source, pure-Rust schematic capture and netlisting application, compatible
with the xschem file formats, with a modern egui/wgpu UI, a web build, and an AI-first design:
anything a person can do, an agent can do.

License: **GPL-3.0-or-later**. Every source file starts with `SPDX-License-Identifier: GPL-3.0-or-later`.
SchemCraft is an independent project, not affiliated with or endorsed by xschem or its author.

## Start every session here
1. Read `plan/STATUS.md` (current milestone, next task), then the task in `plan/execution-plan.md`
   and the relevant `plan/architecture.md` section.
2. Decisions live in `plan/adr/`. Add a new ADR for any decision that changes them.

## Reference material (`reference/`, gitignored, never committed)
- `reference/xschem` (GPL-2.0-or-later): **may be read and ported** (file formats, netlisting,
  label/bus expansion). Every ported area is recorded in `ATTRIBUTION.md`; keep the original
  copyright notices on ported code.
- `reference/solidcraft` (MIT OR Apache-2.0): convention reference (workspace layout, command
  model, xtask checks). Copied code is noted in `ATTRIBUTION.md`. Never copy its icons or artwork.
- Do not pull in code from other projects unless its license is GPL-3.0-or-later compatible and
  it is recorded in `ATTRIBUTION.md`. Never copy code, icons, text or workflows from commercial
  EDA products.

## Never crash
A crash loses a user's design. This outranks feature work.
- No `unwrap()`, `expect()`, `panic!`, `todo!`, `unimplemented!`, `unreachable!` in non-test code;
  no `unsafe` (`unsafe_code = "forbid"`). Every production crate root carries the matching
  `#![deny(clippy::...)]`.
- Errors are `Result<T, E>` with `?`. An unfinished feature returns `not supported yet: ...`.
- Input-derived numbers and files are hostile: `get()` not `[i]`, finite checks, size and loop caps.
- Every crash fix lands with a regression test in the existing test file for its area.

## Non-negotiables
- **Everything is a command.** User-visible behaviour is a command in `crates/engine` with our own
  dotted lower-case id `<area>.<thing>[.<variant>]` (e.g. `sch.wire.add`), a JSON parameter schema,
  an `enabled` rule and tests. UI, CLI, MCP, scripts and the assistant all call the same commands.
- **Programmatic calls never open dialogs.** `Session::execute` takes JSON and returns JSON.
- **AI edits are safe.** An agent's edits are one grouped undo step. A **review-before-apply** mode
  (user setting) shows the pending edit as a diff and waits for approval. Agents also get
  read commands (list instances, net connectivity, ERC, render/screenshot) so they can verify.
- **Bring your own agent.** The assistant connects to agents/endpoints the user already has; it
  bundles no model and no key. API keys go in the system keychain. A master "Show AI features"
  switch removes all AI UI and starts nothing when off.
- **Web is first-class.** `core`, `format`, `netlist`, `engine` compile to
  `wasm32-unknown-unknown`: no threads, filesystem or process spawning there; platform services sit
  behind traits.
- **Layering** is enforced by `cargo xtask layers`: L0 `core` → L1 `format`, `netlist` →
  L2 `engine` → L3 `ui-egui` → apps. Nothing below L3 depends on egui/eframe/winit/wgpu/rfd.
  The UI crate is swappable.
- **Compatibility first.** `.sch`/`.sym` files written by xschem must round-trip; netlists are
  diffed against xschem output in tests.
- **Dependencies** must be GPL-3.0-or-later compatible (MIT/Apache-2.0/BSD/MPL). Never add
  GPL-2.0-only, proprietary or non-commercial code. Check with `cargo deny` once configured.

## Checks before finishing a task
`cargo fmt --check && cargo clippy --workspace --all-targets && cargo test --workspace && cargo xtask layers`
