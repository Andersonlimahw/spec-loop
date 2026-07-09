# Product Context

SpecLoop project context lives in `.specloop/`.

## Cold Context

Stable context that can be cached or loaded once:

- product overview;
- specs;
- business rules;
- acceptance criteria;
- critical flows;
- scenario templates;
- loop runbooks.

## Hot Context

Run-specific context:

- target URL;
- current objective;
- recent findings;
- screenshots and traces;
- console/network summaries;
- run scratchpad.

The split keeps prompts small and reduces repeated work across runs.
