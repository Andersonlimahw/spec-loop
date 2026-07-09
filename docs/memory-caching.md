# Memory And Caching

SpecLoop applies a cold/hot context split.

## Cold Cache

Cold context is stable and content-addressed. It can be reused across runs until
one of these changes:

- prompt version;
- spec hash;
- adapter schema hash;
- project profile hash;
- business-rule hash.

## Hot Retrieval

Hot context is selected per run under a budget. It should contain only relevant
facts and summarized evidence, not raw transcripts or full DOM dumps.

## Durable Memory

Durable memory should be typed:

```text
fact + source + confidence + ttl + created_at
```

Secrets, cookies, raw traces, and full browser sessions must not become durable
memory.
