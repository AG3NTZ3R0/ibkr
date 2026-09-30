# Quickstart: Validate Complete Endpoint Wrappers

**Feature**: [spec.md](spec.md) | **Contract**: [contracts/public-api.md](contracts/public-api.md) |
**Audit**: [data-model.md](data-model.md)

## Prerequisites

- Rust toolchain supporting edition 2024 (`cargo --version`).
- Checked out on `001-complete-endpoint-wrappers`.
- Live check only: a running, logged-in Client Portal Gateway at `https://localhost:5000`, or
  OAuth credentials as described in the README.

## 1. Unit suite (primary evidence, SC-004)

```bash
cargo test --all-features
```

Expected: every test passes, including, per module:

| Module | Proves |
| --- | --- |
| `history` | `direction` emits `-1` / `1` when set and is absent when unset (US1 scenarios 1–2); `Source::Last` emits `Last`; decodes `examples/history.last.json` |
| `search` | each of `more`, `fund`, `fundFamilyConidEx`, `pattern`, `referrer` emitted when set, absent when unset; decodes `examples/secdef_search.json` with `opt`, `war`, `bondid`, `issuers`, and a section `conid` populated |
| `ssodh::init` | decodes `examples/ssodh_init.json` with `established`, `mac`, `server_info` populated |
| `tickle` | decodes `examples/tickle.success.json` (`sso_expires`, `user_id`, `collission`, `hmds.error`, `iserver.auth_status.established`) and `examples/tickle.fail.json` (`error`) |
| `live_session_token` | decodes `examples/live_session_token.json` |

The fixtures' origin and the audit they satisfy are in [research.md § R9](research.md) and
[data-model.md](data-model.md).

## 2. Compile check for callers (FR-011)

```bash
cargo build --all-targets
```

Expected: `src/main.rs`, `examples/search.rs`, and the README snippets compile once the new
fields are spelled out as `None`. They MUST NOT set any new parameter (FR-012).

## 3. Rule is visible (US3, SC-005)

```bash
grep -n "in full" README.md
grep -rn "Docs: <https://www.interactivebrokers.com/docs/web-api/api-reference/" src/
```

Expected: the README states the full-coverage rule, and each of the five endpoint modules
prints one reference link. Each link returns 200:

```bash
grep -rhoE 'https://www.interactivebrokers.com/docs/web-api/api-reference/[^>]+' src/ \
  | xargs -n1 curl -sSL -A Mozilla/5.0 -o /dev/null -w '%{http_code} %{url_effective}\n'
```

## 4. One confirming live run (optional, SC-001)

At most **one** run, per Principle IV. Against the gateway, request TSLA daily bars forward from
an early `start_time` with `direction: Some(Direction::Forward)`. Expected: a 2xx response whose
first bar is on or after `start_time`, with no backward probing. If IBKR returns an intermittent
5xx, report the check as unconfirmed; MUST NOT retry in a loop.
