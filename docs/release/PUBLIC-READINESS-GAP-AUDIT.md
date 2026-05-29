---

---

### Dependabot alert triage

GitHub reported two high-severity alerts:

| Alert | Package | Manifest | Classification | Public-readiness action |
|---|---|---|---|---|
| Anthropic's MCP TypeScript SDK has a ReDoS vulnerability | `@modelcontextprotocol/sdk` | `docs/wiki/deepwiki-dump-monad-workspace/package.json` | Imported DeepWiki documentation/tooling artifact, not Monad runtime | Remove or quarantine imported npm manifest/dependency metadata from first-party docs tree. |
| MCP TypeScript SDK does not enable DNS rebinding protection by default | `@modelcontextprotocol/sdk` | `docs/wiki/deepwiki-dump-monad-workspace/package.json` | Imported DeepWiki documentation/tooling artifact, not Monad runtime | Remove or quarantine imported npm manifest/dependency metadata from first-party docs tree. |

Decision: these alerts are real public-readiness blockers while the manifest remains committed, but they do not indicate a vulnerability in Monad's Rust CLI runtime. The repository should not commit generated/imported DeepWiki npm dependency metadata under `docs/`.

