# Demo — Automated Instrument Validation

Run through every Omegon capability in sequence. Narrate each phase so the operator can watch the instrument panel respond. Do NOT pause between phases — run the entire sequence automatically.

## Phase 1: Baseline Check
Print: "═══ PHASE 1: BASELINE — all instruments should be dim navy ═══"
Print: "👁 Watch: all four instruments (context/tools/thinking/memory) should be near-invisible"
Sleep 3 seconds.

## Phase 2: Context Fill
Print: "═══ PHASE 2: CONTEXT FILL — Perlin flow brightens ═══"  
Print: "👁 Watch: context instrument lights up, flow speeds up"
Read every file in this project (README.md, Cargo.toml, src/lib.rs, src/main.rs, AGENTS.md). Summarize each file in one sentence.

## Phase 3: Tool Execution
Print: "═══ PHASE 3: TOOL EXECUTION — Lissajous curves spike ═══"
Print: "👁 Watch: tools instrument flashes with each tool call, curves appear"
Create a file `src/config.rs` containing:
```rust
/// Configuration for the demo project.
pub struct Config {
    pub name: String,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { name: "demo".into(), verbose: false }
    }
}
```
Then create `tests/integration.rs` containing:
```rust
use omegon_demo::greet;

#[test]
fn greeting_integration() {
    let result = greet("demo");
    assert!(result.contains("demo"));
}
```
Then run `cargo test` to verify everything compiles and passes.

## Phase 4: Extended Thinking  
Print: "═══ PHASE 4: THINKING — Plasma waves ripple with ignition effect ═══"
Print: "👁 Watch: thinking instrument activates — color ramps first, speed follows"
Without changing the thinking level (use whatever is set), analyze: What are the three most important architectural decisions visible in this project's structure? Consider the workspace layout, module organization, and test strategy. Provide a brief but thoughtful analysis.

## Phase 5: Memory Operations
Print: "═══ PHASE 5: MEMORY — Waterfall cascades with CRT glitch characters ═══"
Print: "👁 Watch: memory waterfall lights up on each operation, different patterns per op"

Store these facts:
1. "The demo project is a minimal Rust workspace used for Omegon TUI instrument validation"
2. "The demo follows a 9-phase sequence: baseline, context, tools, thinking, memory, design, burst, focus, cleanup"

Then recall: "What do you know about the demo project?"

Then search episodes: "demo"

## Phase 6: Design Tree
Print: "═══ PHASE 6: DESIGN TREE — Dashboard sidebar updates ═══"
Print: "👁 Watch: tools instrument fires, dashboard sidebar shows new node"

Create a design node with id "demo-validation", title "Demo validation results", status "exploring", overview "Captures the results of running the demo sequence against the Omegon TUI".

Add a question: "Did all four instruments respond correctly to their telemetry sources?"

Add a decision: title "Instruments operational", status "decided", rationale "All phases completed successfully — each instrument responded to its telemetry source with the expected visual behavior."

## Phase 7: Burst Activity
Print: "═══ PHASE 7: BURST — All instruments active simultaneously ═══"  
Print: "👁 Watch: ALL FOUR instruments should be visibly active at the same time"

Do all of these in rapid succession:
1. Read README.md
2. Store a fact: "Burst phase exercises all instruments simultaneously"
3. Write a file `docs/VALIDATION.md` with content "# Validation\nDemo passed at $(date)"
4. Run `cargo test`
5. Recall: "What facts exist about the demo?"

## Phase 8: Focus Mode
Print: "═══ PHASE 8: FOCUS MODE — Instrument panel disappears and returns ═══"
Print: "👁 Watch: type /focus to hide instruments, /focus again to restore"
Print: "⏸ This phase is manual — type /focus twice to validate the toggle"

## Phase 9: Cleanup
Print: "═══ PHASE 9: CLEANUP — Return to idle state ═══"
Print: "👁 Watch: memory waterfall fires during cleanup, then all instruments settle to idle"

Archive any facts stored during this test that contain "demo".
Delete the file `docs/VALIDATION.md`.
Remove the design node "demo-validation" if it exists (set status to deferred).

Print: "═══ DEMO COMPLETE ═══"
Print: "Review the checklist in README.md and check off each item that passed."
