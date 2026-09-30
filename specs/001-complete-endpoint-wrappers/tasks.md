---

description: "Task list for Complete the Endpoint Wrappers"
---

# Tasks: Complete the Endpoint Wrappers

**Input**: Design documents from `specs/001-complete-endpoint-wrappers/`
([Pitch #9](https://github.com/AG3NTZ3R0/ibkr/issues/9))

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md),
[data-model.md](data-model.md) (the audit), [contracts/public-api.md](contracts/public-api.md),
[quickstart.md](quickstart.md)

**Tests**: REQUIRED. FR-008 and constitution Principle IV require each endpoint to test request
construction and to decode its documented example. Each story writes its tests first, and they
MUST fail (by assertion or by compile error) before the implementation lands.

**Organization**: Tasks are grouped by user story so each story can be built and verified on its
own.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependency on an incomplete task)
- **[Story]**: The user story the task belongs to (US1, US2, US3)

## Conventions for every task

- Tests MUST stay inline in each module's existing `#[cfg(test)] mod tests`. No `tests/`
  directory is created.
- Fixtures MUST be pasted inline as the module's `SAMPLE`-style `r#"…"#` consts, copied verbatim
  from `specs/001-complete-endpoint-wrappers/contracts/examples/`. The crate MUST NOT
  `include_str!` from `specs/`.
- New request fields MUST be `Option<_>` and pushed onto `Endpoint::query` only when `Some`,
  following the existing `if let Some(..) = .. { q.push(..) }` pattern (FR-004).
- New request enums MUST derive `Debug, Clone, Copy, PartialEq, Eq` and expose
  `pub fn as_str(self) -> &'static str`, matching `BarSize`/`Source`/`SecType` (FR-005).
- New response fields MUST be `Option<_>` or `Vec<_>` under the struct's existing
  `#[serde(rename_all = "camelCase", default)]`. New response structs MUST derive
  `Debug, Clone, Default, Deserialize` with the same attribute (FR-006).
- No existing response field MAY be removed (FR-007). No new dependency MAY be added
  (Principle III).
- Comments MUST carry only gotchas (Principle III). The ones called for are named in the tasks
  below.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm the starting point before any change.

- [X] T001 Confirm the branch `001-complete-endpoint-wrappers` is based on a freshly fetched
  `origin/main` (`git fetch origin && git merge-base --is-ancestor origin/main HEAD`) and that
  `cargo test --all-features` passes at the root `Cargo.toml` before any edit, to establish the
  baseline

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: None. Every change is additive inside an existing module, and no shared type or
infrastructure blocks the stories. The one cross-module dependency (tickle reuses
`ssodh::init::Response`, research § R7) is ordered inside User Story 2.

**Checkpoint**: The baseline passes, so user story work can begin.

---

## Phase 3: User Story 1 - Page history forward (Priority: P1) 🎯 MVP

**Goal**: A caller sets `direction: Some(Direction::Forward)` on `history::Request` and pages
forward from `start_time` to now (FR-001, SC-001).

**Independent Test**: `cargo test --all-features iserver::marketdata::history` passes. A request
with `Direction::Forward` emits `("direction", "1")`, `Direction::Backward` emits
`("direction", "-1")`, and a request with `direction: None` emits no `direction` pair.

### Tests for User Story 1 ⚠️

- [X] T002 [US1] In `src/iserver/marketdata/history.rs` tests, add
  `query_emits_direction_only_when_set`: `Some(Direction::Forward)` MUST yield
  `("direction","1")`, `Some(Direction::Backward)` MUST yield `("direction","-1")`, and `None`
  MUST yield no key `direction` (US1 scenarios 1–2). Extend
  `query_includes_required_and_set_optionals` to set `source: Some(Source::Last)` and assert
  `("source","Last")`
- [X] T003 [US1] In `src/iserver/marketdata/history.rs` tests, replace the `SAMPLE` const with
  the contents of `specs/001-complete-endpoint-wrappers/contracts/examples/history.last.json`
  (research § R9) and rewrite `decodes_official_sample` to assert the new example's values,
  including `data.len() == 4`, and the first bar's `o`/`c`/`t`

### Implementation for User Story 1

- [X] T004 [US1] In `src/iserver/marketdata/history.rs`, add
  `pub enum Direction { Backward, Forward }` with `as_str` → `"-1"` | `"1"` (data-model § 1).
  Add one gotcha comment only: the reference contradicts itself on whether `Forward` needs
  `startTime`, and the crate enforces neither rule (research § R4)
- [X] T005 [US1] In `src/iserver/marketdata/history.rs`, add
  `pub direction: Option<Direction>` to `Request` between `outside_rth` and `source`, and in
  `Endpoint::query` push `("direction", d.as_str())` only when `Some`, placed before the
  `source` push (contracts/public-api.md)
- [X] T006 [US1] In `src/iserver/marketdata/history.rs`, replace `Source::Trades` (wire
  `"Trades"`) with `Source::Last` (wire `"Last"`), so the set is exactly
  `{Last, Midpoint, BidAsk}` → `"Last"` | `"Midpoint"` | `"Bid_Ask"` (research § R5)
- [X] T007 [US1] In `src/main.rs`, add `direction: None` to the `history::Request` literal so
  it compiles. It MUST NOT set `Some(..)` (FR-012)
- [X] T008 [US1] Run `cargo test --all-features` and `cargo build --all-targets`. Both MUST pass
  with T002–T003 green

**Checkpoint**: Forward paging is reachable. This is the MVP, and it closes the TSLA failure.

---

## Phase 4: User Story 2 - Every documented parameter and field is reachable (Priority: P2)

**Goal**: Every parameter and field in the audit (data-model.md) is present on search,
ssodh/init, tickle, and live_session_token (FR-002–FR-008, SC-002–SC-004).

**Independent Test**: `cargo test --all-features` passes. Each module decodes its
`contracts/examples/` fixture with every added field populated, and each added search parameter
is emitted when set and absent when unset.

### Tests for User Story 2 ⚠️

- [X] T009 [P] [US2] In `src/iserver/secdef/search.rs` tests, replace `SAMPLE` with
  `specs/001-complete-endpoint-wrappers/contracts/examples/secdef_search.json` and extend
  `decodes_official_sample` to assert `opt`, `war`, `bondid`, `issuers`, and a `sections` entry
  with `conid` populated. Extend `query_includes_required_and_set_optionals` so each of `more`,
  `fund`, `fundFamilyConidEx`, `pattern`, and `referrer` is asserted present when set, and the
  `bare` request asserts all five absent
- [X] T010 [P] [US2] In `src/iserver/auth/ssodh/init.rs`, add a `#[cfg(test)] mod tests` that
  decodes `specs/001-complete-endpoint-wrappers/contracts/examples/ssodh_init.json` and asserts
  `established`, `mac`, and `server_info` (`server_name`, `server_version`) are populated
- [X] T011 [P] [US2] In `src/tickle.rs`, add a `#[cfg(test)] mod tests` that decodes
  `contracts/examples/tickle.success.json` (asserting `sso_expires`, `user_id`, `collission`,
  `hmds.error`, and `iserver.auth_status.established`) and `contracts/examples/tickle.fail.json`
  (asserting `error`)
- [X] T012 [P] [US2] In `src/oauth/live_session_token.rs`, add a `#[cfg(test)] mod tests` that
  decodes `specs/001-complete-endpoint-wrappers/contracts/examples/live_session_token.json` into
  the private `Response` and asserts all three fields. This is the module's first test
  (data-model § 5)

### Implementation for User Story 2

- [X] T013 [P] [US2] In `src/iserver/secdef/search.rs`, add to `Request`: `more: Option<bool>`
  → `"more"`, `fund: Option<bool>` → `"fund"`, `fund_family_conid_ex: Option<String>` →
  `"fundFamilyConidEx"`, `pattern: Option<bool>` → `"pattern"`, and `referrer: Option<String>` →
  `"referrer"`, each pushed in `Endpoint::query` only when `Some`. Add to `Contract`:
  `fop`, `opt`, and `war`, each `Option<String>` (`;`-separated dates). Add to `Section`:
  `conid: Option<String>`. `restricted` stays `Option<String>` and `Section::symbol` stays
  (research § R6)
- [X] T014 [US2] In `examples/search.rs`, add `more: None, fund: None,
  fund_family_conid_ex: None, pattern: None, referrer: None` to the `search::Request` literal
  (FR-012; depends on T013)
- [X] T015 [P] [US2] In `src/iserver/auth/ssodh/init.rs`, add to `Response`:
  `established: Option<bool>`, `mac: Option<String>` with `#[serde(rename = "MAC")]`,
  `server_info: Option<ServerInfo>`, and `hardware_info: Option<String>` with
  `#[serde(rename = "hardware_info")]`. Add `pub struct ServerInfo { server_name:
  Option<String>, server_version: Option<String> }`. One gotcha comment only: `MAC` and
  `hardware_info` fall outside `camelCase` on the wire
- [X] T016 [US2] In `src/tickle.rs`, add to `Response`: `sso_expires: Option<i64>`,
  `collission: Option<bool>`, `user_id: Option<i64>`, `hmds: Option<Hmds>`,
  `iserver: Option<Iserver>`, and `error: Option<String>`. Add
  `pub struct Hmds { error: Option<String>, auth_status: Vec<serde_json::Value> }` and
  `pub struct Iserver { auth_status: Option<crate::iserver::auth::ssodh::init::Response> }`
  (research § R7–R8). One gotcha comment only: `collission` is IBKR's spelling of the wire key.
  Depends on T015
- [X] T017 [US2] Run `cargo test --all-features` and `cargo build --all-targets`. Both MUST pass
  with T009–T012 green. Then check every **ADD** and **CHANGE** row in
  `specs/001-complete-endpoint-wrappers/data-model.md` against the code (SC-002)

**Checkpoint**: The audit is closed. Every documented parameter and field is reachable.

---

## Phase 5: User Story 3 - Full coverage is the stated rule (Priority: P3)

**Goal**: The README states the full-coverage rule, and every endpoint module links its current
reference page (FR-009, FR-010, SC-005).

**Independent Test**: The two `grep` commands in quickstart.md § 3 find the rule and five links,
and each link returns 200.

### Implementation for User Story 3

- [X] T018 [P] [US3] Replace each module's `Docs:` link with its research § R2 URL:
  `src/iserver/marketdata/history.rs` → `…/api-reference/trading/trading-market-data/get-md-history`;
  `src/iserver/secdef/search.rs` → `…/api-reference/trading/trading-contracts/get-contract-symbols`;
  `src/iserver/auth/ssodh/init.rs` → `…/api-reference/trading/trading-session/initialize-session`;
  `src/tickle.rs` → `…/api-reference/trading/trading-session/get-session-token`. In
  `src/oauth/live_session_token.rs`, add
  `…/api-reference/authentication/oauth-1-0-a/req-live-session-token` and keep the existing
  `compute-live-session-token` guide link. Every `…` is
  `https://www.interactivebrokers.com/docs/web-api`
- [X] T019 [P] [US3] In `README.md`, add one sentence stating that each endpoint is wrapped in
  full, meaning every request parameter and response field in IBKR's Web API reference, and that
  each module links its reference page. The sentence MUST contain the phrase "in full" so
  quickstart § 3 finds it. Add `more: None, fund: None, fund_family_conid_ex: None,
  pattern: None, referrer: None` to the `search::Request` snippet

**Checkpoint**: The rule is visible to the next contributor.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Release versioning and the quickstart validation.

- [X] T020 Set `version = "0.7.0"` in `Cargo.toml` and `ibkr = "0.7"` in the `README.md`
  dependency snippet (FR-011, research § R10)
- [X] T021 Run quickstart.md §§ 1–3: `cargo test --all-features`, `cargo build --all-targets`,
  both `grep` checks, and the `curl` loop over the reference links. Every step MUST pass
- [X] T022 OPTIONAL: run quickstart.md § 4 as at most **one** live run, a TSLA daily request with
  `direction: Some(Direction::Forward)`, from a scratch caller that is not committed (FR-012). An
  intermittent 5xx MUST be reported as unconfirmed and MUST NOT be retried (Principle IV)
  - Confirmed 2026-09-29 in one run: conid 76792991, `start_time` `20250102-00:00:00`, period
    `1w` returned 2xx, symbol TSLA, 3 bars, first `t` 1736173800000 (2025-01-06), on or after
    `start_time`. IBKR echoed `startTime` `20250103-14:30:00`
- [X] T023 Commit with the breaking prefix `feat!:` (Conventional Commits, Principle V), after
  confirming the branch is still based on the current `origin/main`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Empty. It passes once T001 does.
- **US1 (Phase 3)**: Depends on T001 only.
- **US2 (Phase 4)**: Depends on T001 only. It is independent of US1 because it touches different
  files.
- **US3 (Phase 5)**: T018 edits the header lines of files that US1 and US2 also edit, so run it
  after those stories to avoid conflicting edits. T019 is independent.
- **Polish (Phase 6)**: Depends on all stories.

### Within Each Story

- US1: T002–T003 (tests, which fail) → T004 → T005 → T006 → T007 → T008. All but T007 edit
  `history.rs`, so they run in sequence.
- US2: T009–T012 (tests) → T013 → T014; T015 → T016; → T017.
  - **T016 depends on T015** (tickle's `Iserver` embeds `ssodh::init::Response`).
  - T009 and T013 both edit `search.rs`, so write the test first, then implement.

### Parallel Opportunities

- US1 and US2 can proceed at the same time.
- Within US2, the tests T009, T010, T011, and T012 are in four different files.
- Within US2, T013 (search) and T015 (ssodh) can run in parallel. T012 (live_session_token) is
  independent of both.
- In US3, T018 and T019 edit different files.

---

## Parallel Example: User Story 2

```bash
# Tests, one per file:
Task: "T009 search tests in src/iserver/secdef/search.rs"
Task: "T010 ssodh/init decode test in src/iserver/auth/ssodh/init.rs"
Task: "T011 tickle decode tests in src/tickle.rs"
Task: "T012 live_session_token decode test in src/oauth/live_session_token.rs"

# Then implementation:
Task: "T013 search fields in src/iserver/secdef/search.rs"
Task: "T015 ssodh/init fields + ServerInfo in src/iserver/auth/ssodh/init.rs"
# After T015: T016 tickle; after T013: T014 examples/search.rs
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. T001 (baseline).
2. Phase 3 (T002–T008): `direction` ships, `Source::Last` is corrected, and `main.rs` compiles.
3. **Stop and validate**: run the US1 independent test. The fix for the motivating failure is
   now in hand.

### Incremental Delivery

1. US1 → forward paging (MVP).
2. US2 → the audit is closed for the other four endpoints.
3. US3 → the rule and links are visible.
4. Polish → `0.7.0`, quickstart validation, and a `feat!:` commit. Merging to `main` publishes
   it, so the version MUST be correct before merge (Principle V).

The release is one breaking version, so the stories ship together in one PR rather than as
separate releases.

---

## Notes

- Scope is fixed by FR-012: no new endpoints, and no caller moves to a new parameter.
- The audit's deliberate non-changes (research § R6) MUST NOT be "fixed" during implementation:
  `period` stays `Option<String>`, `BarSize` gets no second values, `restricted` stays
  `Option<String>`, the response `direction` stays `Option<i64>`, `chartAnnotations` and
  `Section::symbol` stay, and `diffie_hellman_response` stays.
