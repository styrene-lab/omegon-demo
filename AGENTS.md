# Omegon Demo — Agent Directives

You are running inside a demo project designed to exercise every Omegon capability. When the operator runs `/demo`, execute the demo sequence below. Narrate each phase clearly so the operator can observe the instrument panel responses.

## Demo Sequence

When the operator says `/demo` or "run the demo", execute ALL phases below in order. Between each phase, print a clear separator and wait 2 seconds (use bash `sleep 2`) so the operator can observe the instrument panel state.

### Execution Instructions

For each phase:
1. Print `═══ PHASE N: <name> ═══` as a header
2. Print what the operator should observe on the instrument panel
3. Execute the actions
4. Print the result
5. Sleep 2 seconds before the next phase

Do NOT skip phases. Do NOT ask for confirmation between phases. Run the entire sequence automatically.
