---
title: AI Context Model
status: complete
epic: E18
---

# AI Context Model

Core types:

```text
AiProviderConfig
AiProviderMode
MemoryRecord
MemoryRecordKind
AiContextArtifact
AiContextPlan
AiContextApplyResult
```

The model is provider-agnostic and local-first.

E18 generates context artifacts only. It does not call models or run agents.
