# E20 Supervised Patch Apply Foundation

This generated Monad artifact records that the repository has an E20 patch planning foundation.

Implemented safety posture:

- patch plans are deterministic;
- dry-run output is available before apply;
- conflict checks prevent silent overwrites;
- generated writes require explicit `--yes`;
- user-owned source mutation remains blocked in this foundation slice;
- no remote patch fetching, arbitrary script execution, AI-provider calls, or destructive deletion occur.
