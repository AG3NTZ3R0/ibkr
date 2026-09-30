# Research: Complete the Endpoint Wrappers

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md) | **Date**: 2026-09-29

## R1. Where the reference lives

**Decision**: Audit against `https://www.interactivebrokers.com/docs/openapi/api-reference.json`
(OpenAPI 3.1.0, 178 paths, fetched 2026-09-29, SHA-256
`a970a00cc447e836792d2991ba59827eb9d710c5c35b31a89e685fe89a9b66ff`). Use the rendered reference
page for each operation only for its example responses, which the OpenAPI file omits.

**Rationale**: The URL the pitch and FR-003 name,
`https://ibkrcampus.com/docs/openapi/api-reference.json`, now returns Cloudflare error 1000
("DNS points to prohibited IP"), which is a fault on IBKR's side. IBKR serves the same OpenAPI 3.1
file from its own domain at the path above, and `/docs/web-api/openapi/api-reference.json`
returns identical bytes. Both hosts are IBKR's, so Principle I holds. None of the five
operations carries an `example`/`examples` key in the file. The rendered pages, generated from
the same source, embed named examples for each one.

**Alternatives considered**:
- `https://www.interactivebrokers.com/api/doc.json` (Swagger 2.0, `localhost:5000`): the legacy
  gateway spec. It is older than the 3.1 reference and not the one the pitch names.
- `https://api.ibkr.com/gw/api/v3/api-docs` (OpenAPI 3.0 "IB REST API"): a different API
  surface, not the Web API reference.

## R2. Reference links (FR-010)

**Decision**: Each module's `Docs:` line MUST point to the rendered reference page at its
current path. That path now carries a `/trading/` segment
(`…/api-reference/trading/<tag>/<operation>`) or, for the token, `/authentication/`.

| Endpoint | Reference link |
| --- | --- |
| `/iserver/marketdata/history` | `https://www.interactivebrokers.com/docs/web-api/api-reference/trading/trading-market-data/get-md-history` |
| `/iserver/secdef/search` | `https://www.interactivebrokers.com/docs/web-api/api-reference/trading/trading-contracts/get-contract-symbols` |
| `/iserver/auth/ssodh/init` | `https://www.interactivebrokers.com/docs/web-api/api-reference/trading/trading-session/initialize-session` |
| `/tickle` | `https://www.interactivebrokers.com/docs/web-api/api-reference/trading/trading-session/get-session-token` |
| `/oauth/live_session_token` | `https://www.interactivebrokers.com/docs/web-api/api-reference/authentication/oauth-1-0-a/req-live-session-token` |

**Rationale**: All four links the crate carries today (`…/api-reference/<tag>/<operation>`)
return 404. The paths above come from `https://www.interactivebrokers.com/docs/sitemap.xml`, and
each returned 200. The live session token module keeps its existing link to the
`compute-live-session-token` guide in addition, because that guide documents the DH math the
module implements.

**Alternatives considered**: Linking the OpenAPI `operationId` (for example
`tradingMarketData_getMdHistory`). Nobody can click it, and it tells a contributor less than the
page does.

## R3. The live session token endpoint is in the reference

**Decision**: Audit `/oauth/live_session_token` against its OpenAPI entry
(`tradingOAuth10A_reqLiveSessionToken`) and its rendered example. The FR-003 fallback is not
needed.

**Rationale**: The spec's edge case assumed this endpoint was absent from the file, but it is
present as `/v1/api/oauth/live_session_token`. The schema and the example disagree. The schema
names the DH field `diffie_hellman_challenge`, while the rendered example and the
`obtain-live-session-token-signature` guide name it `diffie_hellman_response`, which is the name
the crate decodes and the live gateway sends. The spec's edge case on prose-versus-example drift,
together with Principle I (the live gateway settles decoding), keeps `diffie_hellman_response`.
Nothing is missing: the request's one documented parameter, the `Authorization` header, is
already sent, and all three response fields are decoded.

## R4. `direction` on history (FR-001)

**Decision**: Add a new closed enum `history::Direction { Backward, Forward }` with wire values
`"-1"` and `"1"`, exposed as `Request::direction: Option<Direction>`.

**Rationale**: The reference enumerates `["-1", "1"]` (default `-1`). Variant names describe the
behavior, so a caller can find forward paging without knowing the wire code, which satisfies
User Story 1 scenario 3. The reference contradicts itself on when `1` is allowed:
`startTime` says that without it "direction must be omitted or 1", while `direction` says `1` is
"only supported when startTime is included". The crate does not enforce either rule, just as it
does not enforce the bar-versus-period matrix today. Callers see the gateway's verbatim error
(Principle II).

**Alternatives considered**: `Option<i8>`, which is free-form and violates FR-005, and
`Direction::Minus1`/`Plus1`, which only restates the wire value.

## R5. `source` values

**Decision**: Replace `Source::Trades` (wire `"Trades"`) with `Source::Last` (wire `"Last"`).
The closed set becomes `{BidAsk, Last, Midpoint}`.

**Rationale**: The reference enumerates `Bid_Ask`, `Last`, `Midpoint` (default `Last`) in the
parameter's description. `Trades` appears only in the prose describing a bar type ("Last or
Trades data"), never as an allowed value. FR-005 requires the closed set of documented values,
and Principle II forbids modeling beyond the documented shape. The release is already breaking
(FR-011), so the rename adds no further version cost. No in-repo caller uses `Source::Trades`.

**Alternatives considered**: Keep `Trades` and add `Last`. That would ship an undocumented value
alongside its documented synonym.

## R6. Findings kept as they are

These are deliberate non-changes, recorded so the audit is complete:

- **history `bar`**: The reference lists units (`S`, `min`, `h`, `d`, `w`, `m`), not an
  enumeration of values. `BarSize` stays as is. Second-width bars are documented as a unit, but
  the reference names no second values to type, and inventing them would break Principle I. This
  is recorded as an open gap, not closed here.
- **history `period`**: The reference marks it `required: true` with `default: "1d"`.
  `Option<String>` stays, because the parameter is not missing and tightening it is not part of
  this audit.
- **history response `direction`**: The schema types it as string enum `"-1"`, but every example
  sends the number `-1`. `Option<i64>` stays (live gateway plus example).
- **history `chartAnnotations`**: The reference omits it, but it stays under FR-007.
- **search `symbol`**: GET marks it optional and the POST body marks it required. `String` stays;
  a symbol-less search is not a use the crate exposes.
- **search `restricted`**: The schema types it `boolean | null`. The live gateway sends
  comma-separated secTypes and the example sends `null`. `Option<String>` stays, keeping the live
  divergence (Principle II exception).
- **search `Section::symbol`**: Neither the schema nor the example documents it, but it stays
  under FR-007.
- **ssodh/init request `publish`/`compete`**: Both are present. They stay `bool`, because the
  crate always sends both and the audit adds only missing parameters.

## R7. Shared session-status shape

**Decision**: The `brokerageSessionStatus` schema is the body of `ssodh/init` and also
`iserver.authStatus` inside `tickle`. Tickle reuses `ssodh::init::Response` for it rather than
defining a second copy.

**Rationale**: The reference defines the shape once under one title, and a single type keeps the
two in step. This follows the lean-code principle.

**Alternatives considered**: A new shared `SessionStatus` type re-exported from both modules. It
renames a public type for no caller benefit.

## R8. Tickle's two-variant response

**Decision**: Keep one `tickle::Response` struct and add the failed variant's `error` field as
an optional field.

**Rationale**: The reference's `oneOf` is `{successfulTickleResponse, failedTickleResponse}`, and
both variants are all-optional objects with disjoint keys. One all-optional struct decodes both
without an untagged enum. `hmds.authStatus` is documented as an array of "Any type", so it is
typed `Vec<serde_json::Value>`; `serde_json` is already a dependency. The misspelled
`collission` is IBKR's wire key and is kept verbatim.

## R9. Examples used as test fixtures (FR-008)

**Decision**: Each module's test decodes the current documented example taken from its rendered
page. The history fixture is replaced with the current `Last` example (2025 AAPL, four bars).

**Rationale**: The examples the crate tests today predate the reference. The named examples wrap
the body in `{"success":{"value":…}}`, which is the docs generator's framing, not the wire. The
fixtures use the inner value. The search example also documents `conid` inside a `sections`
entry (the CFD section), so `Section::conid` is added under the spec's rule that "a field
documented in either [prose or example] is optional".

## R10. Version

**Decision**: Set `0.6.0 → 0.7.0` and use the commit prefix `feat!:`.

**Rationale**: Under pre-1.0 SemVer as Cargo applies it, a breaking change bumps the minor
version. New public fields break struct-literal callers, and `Source::Trades` is removed
(Principle V).
