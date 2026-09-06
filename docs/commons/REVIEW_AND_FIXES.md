# commons_board — review, fixes, and deploy blockers

**Reviewer:** MIRRA Reviewer (Grok Bot)  
**Date:** 2026-09-06 (PT)  
**Tree:** `/workspace/mirra-commons-attach/commons_board/`  
**Spec:** `docs/MIRRA_COMMONS_V0_SECURITY_SPEC.md`  
**Deploy:** **NOT performed.** Do not deploy until blockers below are cleared by the owner / Policy Gate.

## Local verification

- Initial `cargo test` failed on this box: rustc **1.85.0** vs transitive `psm 0.1.32` / `ar_archive_writer 0.5.3` requiring **1.88**.
- Mitigation: `cargo update -p psm --precise 0.1.21` (lockfile updated). Prefer pinning in CI or upgrading rustc for wasm release builds.
- After fixes: **38 tests passed, 0 failed** (`cargo test`).

## Critical findings fixed

| Sev | Finding | Fix |
| --- | --- | --- |
| **Critical** | Quarantine inverted vs M-6: only root was inserted into quarantine; all other principals could post freely. Root could not post and could not self-admit → deadlock. | Switched to **admitted allowlist** (default-deny). Init admits installer root. `is_quarantined = !is_admitted`. |
| **High** | `StableLog::init` used the **same** `MemoryId` twice (index/data collision risk). | Split `MEMORY_ID_LOG_INDEX` / `MEMORY_ID_LOG_DATA`; renumbered other memory IDs. |
| **High** | `get_all_posts` had **no ACL** (any caller could read full log). | Restricted to warden/root; others get empty vec. |
| **High** | `change_role` mishandled Root/FreezeAuthority (treated as warden); Root role did not update `meta.root`. | Explicit match arms; added `freeze_authority` on `Meta`. |
| **Med** | Freeze only checked root; FreezeAuthority role unused. | `freeze()` allows freeze_authority or root; **unfreeze remains root-only**. |
| **Med** | Rate limit charged before quarantine rejection. | Quarantine check runs before rate-limit mutation. |
| **Med** | Anonymous installer could become root and brick writes. | `init` traps if caller is anonymous. |
| **Low** | `last_chain_hash` scanned full log. | Cache `meta.tip_hash` on append. |
| **Low** | No body size caps (DoS / stable-memory blowup). | Caps: title 512, text 8KiB, refs 32, url/quote limits. |

## Spec honesty (unchanged / deferred)

- v0 signature channel remains **ICP caller attestation**, not in-canister ED25519 (C-1 / T-12).
- Policy Gate is **not** in this canister (proposals only).
- Link fetcher / SSRF (C-9) out of canister scope.
- No PocketIC upgrade integration test; stable wrappers unit-tested only.
- Unit tests still largely exercise types/logic offline — not a full canister state-machine suite for update paths.

## Remaining deploy blockers (do not deploy yet)

1. Owner / Policy Gate explicit GO (spec: message ≠ authority).
2. Confirm wasm32 toolchain (rustc ≥ needed for release deps) and rebuild release wasm; re-hash.
3. `dfx` project wiring, controllers (non-anonymous), freeze key ceremony.
4. PocketIC or replica tests for update methods under real callers.
5. Human review of role model + admitted-allowlist migration (breaking vs prior inverted quarantine).
6. CI pin for `psm` / rustc so builds are reproducible.
7. No secrets, production network, or MIRRA core coupling in this step.

## Files touched

- `src/lib.rs` — security/logic fixes above
- `src/commons_board.did` — `ReplayState.admitted`
- `tests/freeze.rs` — Meta fields
- `Cargo.lock` — psm pin

## Handoff

Ownership of this package and further Commons work is handed to **MIRRA Commons** agent. MIRRA Reviewer awaits further user instructions; no deploy from this session.
