# MIRRA v1 frozen protocol

## Arithmetic

| Field | Frozen value |
|---|---:|
| Contract | `mirra.v1-pilot` |
| Scale | `S = 2^32 = 4294967296` |
| Experts | `2 <= K <= 64` |
| Learning rate | `eta = 1/8`, encoded as `536870912` Q32.32 |
| Loss | unsigned Q32.32, inclusive `[0,S]`; smaller is better |
| Exponential window | `[-16S,0]` |
| Fixed-share floor | `4096` units per expert |
| Rounding | round-to-nearest, ties-to-even |

Each correction recenters losses by their minimum, accumulates the resulting
penalty gap in checked Q64.64 `i128`, and recenters cumulative penalties again.
Readout converts penalty gaps once to Q32.32, evaluates the integer-only P1
exponential, then uses floor-Hamilton allocation. Weights always sum exactly to
`S`; ties are broken by ascending canonical expert ID.

## Canonical identities

An expert is the SHA-256 digest of a domain separator followed by length-prefixed
owner principal bytes, `system_id`, and `version`:

```text
SHA256("mirra.expert.v1\\0" || lp(owner) || lp(system_id) || lp(version))
```

Lengths are unsigned 32-bit big-endian. `system_id` is 1–128 bytes and accepts
lowercase ASCII letters, digits, `-._:/`; `version` is 1–64 bytes and accepts
lowercase ASCII letters, digits, `-._+`. Both must begin and end with an
alphanumeric character. The registry call is itself retry-safe.

## Correction identity and retry contract

The global event key is domain-separated SHA-256 over length-prefixed
`metric_id` and `external_event_id`. It deliberately excludes the current writer
so writer rotation cannot duplicate an event.

Losses are sorted by expert ID before hashing. An identical retry returns the
original sequence, correction ID, state root, and weights with `replayed=true`.
A reused event key with different canonical losses is rejected without mutation.
Authorization is checked before receipt retrieval.

## Authorization

| Bit | Capability |
|---:|---|
| `1` | record corrections |
| `2` | register and configure experts before lock |
| `4` | reserved auditor role |
| `8` | governance administration |

Only the active governance principal with bit 8 can change roles, pause the
canister, or nominate a successor. Governance transfer requires the successor
to accept; acceptance revokes every role from the former governance principal
and grants all roles to the successor. Every governance mutation appends a
durable audit event.

The active expert set locks on the first accepted correction. This prevents a
change in vector meaning after sequence 1.

## Stable schema

State is stored directly through the stable-memory manager. No unbounded heap
snapshot or full-log replay is required during upgrade.

| Memory ID | Structure |
|---:|---|
| 0 | configuration and state root |
| 1 | correction sequence |
| 2 | expert registry |
| 3 | Q64.64 penalties |
| 4 | correction log |
| 5 | idempotency receipts |
| 6 | writer roles |
| 7 | governance audit events |
| 8 | governance event sequence |

Schema version 1 is checked at open and after upgrade. `post_upgrade` restores
the certified state root from stable memory. Pagination is bounded to 100 expert
records per query and the pilot registry is capped at 4,096 identities; the
active set remains capped at 64.
