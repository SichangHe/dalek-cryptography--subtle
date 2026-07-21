# Choice mask proof

- target: the `u8` instance of integer select, assign, and swap
- domain: `Choice` values `0` and `1`
- model: `0u8.wrapping_sub(choice)` has the same mask bits as production's signed negation and cast on this domain
- result: each operation returns the selected value, and swap either preserves or exchanges its inputs
- scope: functional bit-mask semantics only; verification does not establish compiler-level constant-time behavior
- verifier: `/ssd1/sichangheagent/work_logs/private_artifacts/midas_lex_12096/toolchain/verus-2026-03-17/verus`
- version: `0.2026.03.17.a96bad0`
- SHA-256: `9a675a675ad2e37e9d51d1fadcf3e2d8377e413f7d915665d629e30e5d92c6ed`
- `staged_in_artifact_root=false`
- reuse rationale: the 783 MiB verifier distribution already resides on the same filesystem, so the experiment reuses it without duplicating the toolchain in the artifact root
- check: `/ssd1/sichangheagent/work_logs/private_artifacts/midas_lex_12096/toolchain/verus-2026-03-17/verus --no-cheating verus/choice_mask.rs`
