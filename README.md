# Omegon Demo Project

A test harness for manually exercising every Omegon TUI capability against a live LLM session. Run each prompt in sequence and observe the instrument panel response.

## Quick Start

```bash
cd test-project
./demo
```

That's it. The demo script finds Omegon, launches it with the demo prompt pre-loaded, and the 9-phase sequence runs automatically against a live LLM. Watch the instrument panel at the bottom of the screen.

### Manual Mode

To run individual phases or explore interactively:

```bash
cd test-project
omegon
```

Then paste any prompt from the sequence below.

## Demo Sequence

Run each prompt below in order. After each one, observe the expected instrument behavior and check the box.

### Phase 1: Baseline (instruments at idle)

On launch, all four instruments should be dim navy — near-invisible. The footer shows the split panel with text cards left, instruments right.

- [ ] All four instruments labeled: context, tools, thinking, memory
- [ ] Context shows 0%, tools 0%, thinking 0%, memory 0%
- [ ] Waterfall (memory) shows sparse dim glitch characters scrolling slowly
- [ ] Perlin (context) shows subtle dark flow pattern
- [ ] Footer left side shows model name, auth status, fact count

### Phase 2: Context Fill (sonar lights up)

```
Prompt: "Read every file in this project and summarize what you find"
```

**Expected**: Context % climbs as the agent reads files and generates response. The context (Perlin) instrument brightens from navy toward teal. Speed increases as context fills.

- [ ] Context instrument intensity increases during response
- [ ] Perlin flow speeds up as context % climbs
- [ ] Other instruments remain dim (no tools called yet if agent just reads)

### Phase 3: Tool Execution (tools instrument fires)

```
Prompt: "Create a file called src/lib.rs with a simple hello world function, then create src/main.rs that calls it, then run cargo init and verify it compiles"
```

**Expected**: Multiple tool calls fire in sequence (write, write, bash). The tools (Lissajous) instrument should spike with each tool call — curves appear and rotate, then decay between calls.

- [ ] Tools instrument lights up on each tool call
- [ ] Intensity spikes then decays between calls
- [ ] Context continues climbing
- [ ] Lissajous curves visible and rotating

### Phase 4: Extended Thinking (thinking instrument ripples)

```
Prompt: "/think high"
```
Then:
```
Prompt: "Analyze the architectural tradeoffs between a monorepo and polyrepo approach for a project with 5 Rust crates and 3 TypeScript packages. Consider CI/CD, dependency management, and team scaling."
```

**Expected**: With thinking set to high, the thinking (Plasma) instrument should show active rippling interference patterns. The ignition effect should be visible — color ramps up first, speed follows.

- [ ] Thinking instrument activates with plasma waves
- [ ] Waves start slow (ignition), then accelerate
- [ ] Intensity shows high %
- [ ] After response completes, thinking calms back down

### Phase 5: Memory Operations (waterfall cascades)

```
Prompt: "Store a fact: this test project uses Rust with a workspace layout"
```
Then:
```
Prompt: "Recall everything you know about this project's architecture"
```
Then:
```
Prompt: "Search the archive for any previous decisions about testing strategies"
```

**Expected**: Each memory operation should trigger the waterfall. The CA rule should change per operation type (write vs recall vs archive search). Glitch characters should cascade with color shifting toward teal/amber.

- [ ] `memory_store` triggers waterfall activity
- [ ] `memory_recall` triggers different pattern
- [ ] `memory_search_archive` triggers activity
- [ ] Waterfall decays back to idle between operations
- [ ] Fact count in footer text updates

### Phase 6: Design Tree (verifies dashboard sidebar)

```
Prompt: "Create a design node called 'demo-feature' with title 'Demo feature for TUI validation'. Set it to exploring status."
```
Then:
```
Prompt: "Add a question to demo-feature: 'Does the instrument panel respond correctly to design tree changes?'"
```
Then:
```
Prompt: "Add a decision to demo-feature with title 'Instruments validated' status decided rationale 'All four instruments respond to telemetry correctly'"
```

**Expected**: Dashboard sidebar updates with the new node. Tool calls fire for each design_tree operation. Memory may fire for fact storage.

- [ ] Node appears in dashboard sidebar
- [ ] Tools instrument fires during design_tree calls
- [ ] Memory instrument fires if facts are stored

### Phase 7: Burst Activity (all instruments active)

```
Prompt: "Read all files in this project, store a summary fact for each one, create a new file called ARCHITECTURE.md documenting the project structure, then search memory for any related patterns"
```

**Expected**: All four instruments should be active simultaneously — context filling, tools firing, memory cascading, thinking engaged.

- [ ] All four instruments visibly active at once
- [ ] Different visual characters per instrument are distinguishable
- [ ] Color ramp pushes toward amber on the busiest instruments

### Phase 8: Focus Mode

```
Type: /focus
```

**Expected**: Entire instrument panel disappears. Conversation gets full height.

```
Type: /focus (again)
```

**Expected**: Instrument panel reappears with current state.

- [ ] `/focus` hides instrument panel
- [ ] `/focus` again restores it
- [ ] Toggle is instant — no visible delay

### Phase 9: Cleanup

```
Prompt: "Delete the demo-feature design node and remove any test facts you stored"
```

- [ ] Dashboard sidebar updates
- [ ] Memory waterfall fires during cleanup
- [ ] System returns to near-idle state

## What to Look For

### Good Signs
- Color ramp is visible: navy at idle → teal at moderate → amber at high
- Each instrument has distinct visual character (flow vs curves vs waves vs glitch)
- Speed changes are perceptible (not just color)
- Waterfall scrolls with history trailing upward
- Idle instruments are near-invisible but not fully black

### Bad Signs
- Any instrument is blinding bright at idle
- All instruments look the same (no shape differentiation)
- Waterfall is flickering/spazzing instead of scrolling
- Color never reaches amber even at high activity
- Instruments don't decay back to idle after activity stops
- `/focus` toggle has visible delay or layout jump
