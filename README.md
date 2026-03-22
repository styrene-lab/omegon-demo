# Omegon Demo

An interactive guided tour of Omegon's capabilities. Run `/demo` from any Omegon session, or launch directly:

```bash
./demo
```

## What Happens

The demo walks you through Omegon's key features conversationally:

1. **Orientation** — what the instrument panel is, what each display shows
2. **Tool execution** — watch the Lissajous curves light up as files are read
3. **Code generation** — burst tool activity during rapid file creation
4. **Memory operations** — waterfall cascades as facts are stored and recalled
5. **Design tree** — architectural decisions tracked in the dashboard sidebar
6. **Context awareness** — Perlin flow brightens as context fills during deep analysis
7. **Engine panel** — your cockpit: model, tier, thinking, context at a glance
8. **Focus mode** — toggle instruments on/off with `/focus`
9. **Wrap up** — next steps and how to use Omegon for real work

## Pre-Seeded Data

This project comes with:
- **10 memory facts** about Omegon's architecture and design decisions
- **2 design nodes** (demo-walkthrough, benchmark-framework) showing the design tree in action
- **Rust source files** for the agent to read, analyze, and modify

This gives the demo a "lived-in" feel — the dashboard isn't empty, the memory card shows facts, and the instruments have something to work with.

## Customization

Fork this repo and adapt it for your team:
- Replace `src/` with your own codebase
- Edit `.omegon/prompts/demo.md` to match your workflows
- Add domain-specific memory facts to `.pi/memory/facts.jsonl`
- Add design nodes in `docs/` for your architecture

The framework structure stays the same — the content is yours.

## For Token-Conscious Teams

The demo uses a small context window by default. It demonstrates all capabilities without requiring large token spend. For a full experience, run with `/context legion` before starting.
