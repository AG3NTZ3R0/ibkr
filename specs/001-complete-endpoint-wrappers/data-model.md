# Data Model: Complete the Endpoint Wrappers

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md) | **Research**: [research.md](research.md)

This file is the **audit** (FR-002). For each endpoint it lists what the crate lacks relative to
the reference and the source consulted. The reference file and its hash are in
[research.md § R1](research.md#r1-where-the-reference-lives). Examples come from the rendered
reference page and are saved under [contracts/examples/](contracts/examples/).

Legend: **ADD** is missing and MUST be added. **CHANGE** exists but departs from the reference and
MUST be changed. **KEEP** departs from the reference and stays, with its reason in research.md § R6.

All added request fields are `Option<_>` and are omitted from the query when `None` (FR-004). All
added response fields are `Option<_>` or `Vec<_>` under the struct's existing `#[serde(default)]`
(FR-006). No existing response field is removed (FR-007).

## 1. `GET /iserver/marketdata/history` — `iserver::marketdata::history`

Source: OpenAPI `tradingMarketData_getMdHistory` + rendered examples (`Bid_Ask`, `Last`,
`Midpoint`).

### Request

| Wire name | Reference | Crate today | Action |
| --- | --- | --- | --- |
| `conid` | int64, required | `conid: u64` | — |
| `period` | string, required, default `1d` | `period: Option<String>` | KEEP |
| `bar` | string, required, units S/min/h/d/w/m | `bar: BarSize` | KEEP (no second values enumerated) |
| `exchange` | string | `exchange: Option<String>` | — |
| `outsideRth` | boolean | `outside_rth: Option<bool>` | — |
| `startTime` | string `YYYYMMDD-hh:mm:ss` | `start_time: Option<String>` | — |
| `direction` | enum `"-1"`, `"1"` | — | **ADD** `direction: Option<Direction>` |
| `source` | `Bid_Ask`, `Last`, `Midpoint` | `source: Option<Source>`, values `Trades`/`Midpoint`/`Bid_Ask` | **CHANGE** `Source::Trades` → `Source::Last` (`"Last"`) |

New closed set:

| `Direction` variant | Wire | Meaning (reference) |
| --- | --- | --- |
| `Backward` | `-1` | Data ends at startTime / now, extending back (default) |
| `Forward` | `1` | Data begins at startTime, moving toward now |

### Response

All 23 documented fields and all 6 `Bar` fields are already decoded. `chartAnnotations` stays even
though the reference omits it. Nothing is added. The fixture is replaced with the current example
(`history.last.json`).

## 2. `GET /iserver/secdef/search` — `iserver::secdef::search`

Source: OpenAPI `tradingContracts_getContractSymbols` (GET query and POST body agree) + rendered
example (IBM: two STK entries, one BOND).

### Request

| Wire name | Reference | Crate today | Action |
| --- | --- | --- | --- |
| `symbol` | string | `symbol: String` | KEEP |
| `secType` | enum `STK`, `IND`, `BOND` | `sec_type: Option<SecType>` | — |
| `name` | boolean | `name: Option<bool>` | — |
| `more` | boolean | — | **ADD** `more: Option<bool>` |
| `fund` | boolean | — | **ADD** `fund: Option<bool>` |
| `fundFamilyConidEx` | string | — | **ADD** `fund_family_conid_ex: Option<String>` |
| `pattern` | boolean | — | **ADD** `pattern: Option<bool>` |
| `referrer` | string | — | **ADD** `referrer: Option<String>` |

### Response `Contract`

| Wire name | Reference | Crate today | Action |
| --- | --- | --- | --- |
| `conid`, `companyHeader`, `companyName`, `symbol`, `description`, `sections`, `issuers`, `bondid` | as documented | present | — |
| `restricted` | `boolean \| null` | `Option<String>` | KEEP (live sends secType list) |
| `fop` | `string \| null`, `;`-separated dates | — | **ADD** `fop: Option<String>` |
| `opt` | `string \| null`, `;`-separated dates | — | **ADD** `opt: Option<String>` |
| `war` | `string \| null`, `;`-separated dates | — | **ADD** `war: Option<String>` |

### Response `Section`

| Wire name | Reference | Crate today | Action |
| --- | --- | --- | --- |
| `secType`, `months`, `exchange` | schema | present | — |
| `symbol` | neither | present | KEEP (FR-007) |
| `conid` | example only (CFD section) | — | **ADD** `conid: Option<String>` |

`Issuer` (`id`, `name`) is complete.

## 3. `POST /iserver/auth/ssodh/init` — `iserver::auth::ssodh::init`

Source: OpenAPI `tradingSession_initializeSession` + rendered example. The response schema is
`brokerageSessionStatus`.

### Request

`publish` and `compete` (boolean body fields) are both present. Nothing to add.

### Response

| Wire name | Reference | Crate today | Action |
| --- | --- | --- | --- |
| `authenticated`, `competing`, `connected`, `message`, `fail` | as documented | present | — |
| `established` | boolean | — | **ADD** `established: Option<bool>` |
| `MAC` | string | — | **ADD** `mac: Option<String>` (`#[serde(rename = "MAC")]`) |
| `serverInfo` | object | — | **ADD** `server_info: Option<ServerInfo>` |
| `hardware_info` | string | — | **ADD** `hardware_info: Option<String>` (`#[serde(rename = "hardware_info")]`) |

New struct `ServerInfo` (`BrokerageSessionStatusServerInfo`): `server_name: Option<String>`,
`server_version: Option<String>` (camelCase on the wire).

`MAC` and `hardware_info` fall outside the struct's `rename_all = "camelCase"`, so each needs an
explicit rename. This mirrors `bondid` on search.

## 4. `POST /tickle` — `tickle`

Source: OpenAPI `tradingSession_getSessionToken` (`oneOf` successful / failed) + rendered
examples.

### Request

No documented parameters. Nothing to add (spec edge case).

### Response

| Wire name | Reference | Crate today | Action |
| --- | --- | --- | --- |
| `session` | string | present | — |
| `ssoExpires` | int64, ms | — | **ADD** `sso_expires: Option<i64>` |
| `collission` | boolean (IBKR's spelling) | — | **ADD** `collission: Option<bool>` |
| `userId` | int64 | — | **ADD** `user_id: Option<i64>` |
| `hmds` | object | — | **ADD** `hmds: Option<Hmds>` |
| `iserver` | object | — | **ADD** `iserver: Option<Iserver>` |
| `error` | string (failed variant) | — | **ADD** `error: Option<String>` |

New structs:

- `Hmds` (`SuccessfulTickleResponseHmds`): `error: Option<String>`,
  `auth_status: Vec<serde_json::Value>` (documented as an array of "Any type").
- `Iserver` (`SuccessfulTickleResponseIserver`):
  `auth_status: Option<crate::iserver::auth::ssodh::init::Response>`, the same
  `brokerageSessionStatus` shape (research § R7).

## 5. `POST /oauth/live_session_token` — `oauth::live_session_token`

Source: OpenAPI `tradingOAuth10A_reqLiveSessionToken` + rendered example + the
`obtain-live-session-token-signature` guide.

### Request

`Authorization` header (the only documented parameter) is present. Nothing to add.

### Response (private)

| Wire name | Reference | Crate today | Action |
| --- | --- | --- | --- |
| `diffie_hellman_response` | example + guide; schema names it `diffie_hellman_challenge` | present | KEEP (research § R3) |
| `live_session_token_signature` | string | present | — |
| `live_session_token_expiration` | int64 ms | present | — |

Nothing to add. The module has no tests today, so it gains a decode test of the documented
example (Principle IV, FR-008).

## Audit totals

| Endpoint | Request ADD | Request CHANGE | Response ADD |
| --- | --- | --- | --- |
| history | 1 | 1 | 0 |
| secdef/search | 5 | 0 | 4 |
| ssodh/init | 0 | 0 | 4 (+1 struct) |
| tickle | 0 | 0 | 6 (+2 structs) |
| live_session_token | 0 | 0 | 0 |
