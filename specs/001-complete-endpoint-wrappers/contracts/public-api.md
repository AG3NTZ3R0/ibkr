# Contract: Public API after this feature

The crate is a library, so its contract is its public Rust surface. This file lists every public
item the feature adds or changes. Everything else in the public API is unchanged. Field
semantics are in [data-model.md](../data-model.md).

## Breaking changes (release `0.7.0`, `feat!:`)

1. `history::Request`, `search::Request`, `search::Contract`, `search::Section`,
   `ssodh::init::Response`, and `tickle::Response` gain public fields. Struct-literal
   construction without `..` stops compiling.
2. `history::Source::Trades` is removed and replaced by `history::Source::Last`.

## `ibkr::iserver::marketdata::history`

```rust
pub struct Request {
    pub conid: u64,
    pub bar: BarSize,
    pub period: Option<String>,
    pub exchange: Option<String>,
    pub start_time: Option<String>,
    pub outside_rth: Option<bool>,
    pub direction: Option<Direction>,   // new
    pub source: Option<Source>,
}

pub enum Direction { Backward, Forward }          // new; as_str → "-1" | "1"
pub enum Source { Last, Midpoint, BidAsk }        // Trades → Last; as_str → "Last" | "Midpoint" | "Bid_Ask"
```

Query contract: `direction` is emitted as `("direction", "-1" | "1")` only when `Some`.

## `ibkr::iserver::secdef::search`

```rust
pub struct Request {
    pub symbol: String,
    pub name: Option<bool>,
    pub sec_type: Option<SecType>,
    pub more: Option<bool>,                     // new → "more"
    pub fund: Option<bool>,                     // new → "fund"
    pub fund_family_conid_ex: Option<String>,   // new → "fundFamilyConidEx"
    pub pattern: Option<bool>,                  // new → "pattern"
    pub referrer: Option<String>,               // new → "referrer"
}

pub struct Contract { /* existing */ pub fop: Option<String>, pub opt: Option<String>, pub war: Option<String> }
pub struct Section  { /* existing */ pub conid: Option<String> }
```

## `ibkr::iserver::auth::ssodh::init`

```rust
pub struct Response {
    /* existing: authenticated, competing, connected, message, fail */
    pub established: Option<bool>,
    pub mac: Option<String>,            // wire "MAC"
    pub server_info: Option<ServerInfo>,
    pub hardware_info: Option<String>,  // wire "hardware_info"
}

pub struct ServerInfo { pub server_name: Option<String>, pub server_version: Option<String> }
```

## `ibkr::tickle`

```rust
pub struct Response {
    pub session: Option<String>,
    pub sso_expires: Option<i64>,
    pub collission: Option<bool>,   // IBKR's spelling
    pub user_id: Option<i64>,
    pub hmds: Option<Hmds>,
    pub iserver: Option<Iserver>,
    pub error: Option<String>,      // failed-tickle variant
}

pub struct Hmds { pub error: Option<String>, pub auth_status: Vec<serde_json::Value> }
pub struct Iserver { pub auth_status: Option<crate::iserver::auth::ssodh::init::Response> }
```

All new response structs derive `Debug, Clone, Default, Deserialize` with
`#[serde(rename_all = "camelCase", default)]`, matching the existing response types. All new
request enums derive `Debug, Clone, Copy, PartialEq, Eq` and expose `as_str(self) -> &'static str`,
matching `BarSize`/`Source`/`SecType`.

## `ibkr::oauth::live_session_token`

No public change. Only the module's reference link and tests change.

## Documented examples

Fixtures that the tests MUST decode are saved under [examples/](examples/). The docs generator's
`{"success":{"value":…}}` framing is already removed.
