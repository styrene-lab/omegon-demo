---
title: "Code Generation"
---

Watch how multiple tools stack up during a burst of activity.

Create a new file `src/config.rs` with a `Config` struct (fields: `name: String`, `verbose: bool`) and a `Default` implementation.

Create a new file `src/analysis.rs` with an `analyze(text: &str) -> TextStats` function that counts words, lines, and characters. Include a `#[cfg(test)]` module with at least two tests.

Update `src/lib.rs` to include the new modules (`pub mod config;` and `pub mod analysis;`).

Run `cargo test` to verify everything compiles and tests pass.

After: "The tools list should show 'write' and 'bash' stacked at the top — you can see exactly which tools I used and when. The bars show recency: brighter = more recent."

Type **/next** when ready.
