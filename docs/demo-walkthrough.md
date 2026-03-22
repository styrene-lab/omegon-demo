# Demo walkthrough design

status: decided

## Overview

The omegon-demo project serves as both a visual demo and a QA benchmark foundation. The walkthrough is conversational, not mechanical — the agent guides the operator through each capability as a colleague would.

## Decisions

### Conversational guided tour, not headless smoke test
**Status:** decided
**Rationale:** A headless test runs too fast for the operator to see the instruments respond. A conversational guide pauses between phases, explains what the operator should see, and invites exploration.

### Pre-seeded memory and design nodes
**Status:** decided  
**Rationale:** A cold-start demo with zero facts and no design nodes doesn't showcase the memory system or dashboard. Pre-seeding with 10 facts and a design tree gives the demo a "lived-in" feel from the first frame.

## Open Questions

- Should the demo offer to configure local inference if the platform supports it?
- Should there be a "quick mode" that runs faster for repeat demos?
