# Implementation Plan: Complete the Endpoint Wrappers

**Branch**: `001-complete-endpoint-wrappers` | **Date**: 2026-09-29 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/001-complete-endpoint-wrappers/spec.md`
([Pitch #9](https://github.com/AG3NTZ3R0/ibkr/issues/9))

## Summary

Five endpoint wrappers MUST be brought up to IBKR's Web API reference in full, and full coverage
MUST become the stated rule. The audit ([data-model.md](data-model.md)) was run in this phase
against IBKR's OpenAPI 3.1 reference. It finds:

- 6 request parameters to add (history `direction`; search `more`, `fund`,
  `fundFamilyConidEx`, `pattern`, `referrer`)
- 1 enumerated value to correct (history `Source::Trades` → `Source::Last`)
- 14 response fields to add across search, ssodh/init, and tickle, with 3 small new structs

The approach is additive. Each change follows the pattern already in the module: `Option` request
fields pushed onto the query only when set, closed enums with `as_str`, and all-optional
`#[serde(default)]` response structs. Tests decode the reference's current examples. The README
states the rule, every module links its reference page at its current URL, and the crate
releases as `0.7.0` with `feat!:`.

## Technical Context

**Language/Version**: Rust, edition 2024

**Primary Dependencies**: `reqwest` 0.12 (blocking), `serde`, `serde_json`, `thiserror`. No
dependency is added; `serde_json::Value` for tickle's `hmds.authStatus` uses the existing
`serde_json`.

**Storage**: N/A

**Testing**: `cargo test --all-features`, with per-module `#[cfg(test)]` unit tests decoding
documented examples; at most one live confirming run

**Target Platform**: Any platform `reqwest` + `rustls` builds on; published to crates.io

**Project Type**: Library (plus a demo binary `src/main.rs` and `examples/search.rs`)

**Performance Goals**: N/A (no runtime behavior change beyond extra optional query parameters)

**Constraints**: Official IBKR documentation only (Principle I); model the documented shape and
nothing more (Principle II); no new dependencies (Principle III); one-week appetite (SC-006)

**Scale/Scope**: 5 endpoint modules, README, `Cargo.toml`, 2 in-repo callers updated to compile

No NEEDS CLARIFICATION remain. The three items the pitch left open (enum naming, link form,
version) are resolved in [research.md](research.md) § R4, R2, and R10.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / rule | Gate | Pre-design | Post-design |
| --- | --- | --- | --- |
| I. Official docs are the source | Every added field or parameter traces to IBKR's OpenAPI file or its rendered page on `interactivebrokers.com`; no third-party source | PASS | PASS: reference host moved (R1), still IBKR-owned; every row in data-model.md names its source |
| II. Docs-faithful types | Nothing undocumented added; live divergences kept only where decoding needs them | PASS | PASS: `Source::Trades` (undocumented) removed; `restricted`, `direction` (response), and `diffie_hellman_response` kept as live/example divergences (R3, R6) |
| II. Declarative `Endpoint` | New parameters flow through `Endpoint::query`; no bespoke send paths | PASS | PASS |
| II. Verbatim errors | Tickle's failed variant is decoded, not turned into an `Error` | PASS | PASS: modeled as `error: Option<String>` on the 2xx body (R8) |
| III. Lean and portable | No new dependency; no speculative API (builders, `Default` for requests) | PASS | PASS: `ServerInfo`/`Hmds`/`Iserver` are documented objects; `brokerageSessionStatus` reused, not duplicated (R7) |
| III. Scope | No new endpoints; no caller moved to a new parameter (FR-012) | PASS | PASS: `main.rs`/`examples/search.rs` get `None` for new fields only |
| III. Comments | Only gotchas (`collission` spelling, `MAC`/`hardware_info` wire casing, `direction`'s contradictory startTime rule); link docs instead of restating | PASS | PASS |
| IV. Tested against examples | Each endpoint tests request construction and decodes its documented example | PASS | PASS: `live_session_token` gains its first decode test; history fixture refreshed (R9) |
| IV. One live run | At most one confirming run | PASS | PASS: quickstart § 4 |
| V. SemVer / breaking | Breaking change marked `feat!:`, version bumped before merge | PASS | PASS: `0.6.0 → 0.7.0` (R10) |
| Workflow | Branch from fresh `origin/main`; Conventional Commits; artifacts checked against constitution | PASS | PASS: branch base is `8f990ee` (current `main` head) |

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/001-complete-endpoint-wrappers/
├── spec.md
├── plan.md                  # this file
├── research.md              # Phase 0: reference location, decisions R1–R10
├── data-model.md            # Phase 1: the audit (FR-002), per endpoint
├── quickstart.md            # Phase 1: validation guide
├── contracts/
│   ├── public-api.md        # Phase 1: added/changed public Rust items
│   └── examples/            # documented example responses (test fixtures)
├── checklists/requirements.md
└── tasks.md                 # Phase 2 (/speckit-tasks; not created here)
```

### Source Code (repository root)

```text
Cargo.toml                             # version 0.6.0 → 0.7.0
README.md                              # full-coverage rule; ibkr = "0.7"; search snippet gains new fields
src/
├── iserver/
│   ├── marketdata/history.rs          # + Direction, direction; Source::Trades → Last; link; tests
│   ├── secdef/search.rs               # + 5 request params; + fop/opt/war; Section::conid; link; tests
│   └── auth/ssodh/init.rs             # + established, mac, server_info, hardware_info; ServerInfo; link; tests
├── tickle.rs                          # + sso_expires, collission, user_id, hmds, iserver, error; Hmds, Iserver; link; tests
├── oauth/live_session_token.rs        # reference link; decode test for private Response
└── main.rs                            # direction: None (compile only)
examples/search.rs                     # new search fields: None (compile only)
```

**Structure Decision**: Single library crate. Each endpoint stays in its existing module, and
the tests stay inline in `#[cfg(test)] mod tests`, as they are today. No new module or file
appears under `src/`.

## Implementation Order

1. **history** (P1, US1): add `Direction` and `direction`, rename `Source::Trades` → `Last`,
   refresh the fixture, update `main.rs`. This is the MVP; it closes the failure that prompted
   the pitch.
2. **search**, **ssodh/init**, **tickle** (P2, US2): these are independent of each other. Tickle
   depends on `ssodh::init::Response` gaining its fields first (R7).
3. **live_session_token** (P2): link and decode test only.
4. **README, links, version** (P3, US3): rule sentence, `0.7` snippet, `Cargo.toml` bump.
5. `cargo test --all-features`, then at most one live run (quickstart).

## Risks

- **Reference hosting moved**: the pitch's `ibkrcampus.com` URL fails at IBKR's CDN. The mirror
  on `interactivebrokers.com` is used and hashed (R1). Pitch #10 (caching the reference) is the
  remedy and is out of scope.
- **`Source::Trades` removal**: if the gateway still accepts `Trades`, a caller relying on it
  loses it. The release is breaking anyway, and the reference documents only `Last` (R5).
- **Second-width bars**: documented as a unit but not reachable through `BarSize`. This is
  recorded as an open gap (R6), not solved here.
- **Spec edge-case correction**: the spec assumed `/oauth/live_session_token` was absent from the
  reference file. It is present (R3), so the FR-003 fallback does not apply.

## Complexity Tracking

No constitution violations to justify.
