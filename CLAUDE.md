# Claude Compatibility

SpecLoop is provider-agnostic. These notes exist only to help Claude-compatible harnesses run the same protocol.

- Use `specloop doctor` before a run.
- Use `specloop validate` before accepting generated context.
- Use `specloop scaffold all` to generate optional `.claude/agents/`, `.claude/skills/`, `.claude/commands/`, and `scripts/` assets.
- To test a local app, start it first, then ask the agent to run a `.specloop/scenarios/` file.
- To test a remote app, ask the agent to run the scenario directly and keep browser actions read-only.
