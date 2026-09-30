//! `POST /tickle` — keep the session alive; returns the session id used for WebSocket auth.
//! Docs: <https://www.interactivebrokers.com/docs/web-api/api-reference/trading/trading-session/get-session-token>

use crate::Endpoint;
use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct Request;

impl Endpoint for Request {
    type Response = Response;
    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn path(&self) -> String {
        "/tickle".to_string()
    }
}

/// Fields optional + `serde(default)`, unknown ignored.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Response {
    pub session: Option<String>,
    pub sso_expires: Option<i64>,
    /// IBKR's spelling of the wire key.
    pub collission: Option<bool>,
    pub user_id: Option<i64>,
    pub hmds: Option<Hmds>,
    pub iserver: Option<Iserver>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Hmds {
    pub error: Option<String>,
    pub auth_status: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Iserver {
    pub auth_status: Option<crate::iserver::auth::ssodh::init::Response>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Official example responses from the docs.
    const SUCCESS: &str = r#"{
  "collission": false,
  "hmds": {
    "error": "no bridge"
  },
  "iserver": {
    "authStatus": {
      "MAC": "98:F2:B3:23:BF:A0",
      "authenticated": true,
      "competing": false,
      "connected": true,
      "established": true,
      "message": "",
      "serverInfo": {
        "serverName": "JifN19053",
        "serverVersion": "Build 10.25.0p, Dec 5, 2023 5:48:12 PM"
      }
    }
  },
  "session": "bb665d0f55b6289d70bc7380089fc96f",
  "ssoExpires": 460311,
  "userId": 123456789
}"#;
    const FAIL: &str = r#"{
  "error": "failed to process request"
}"#;

    #[test]
    fn decodes_official_success_sample() {
        let resp: Response = serde_json::from_str(SUCCESS).expect("decode sample");
        assert_eq!(resp.session.as_deref(), Some("bb665d0f55b6289d70bc7380089fc96f"));
        assert_eq!(resp.sso_expires, Some(460311));
        assert_eq!(resp.user_id, Some(123456789));
        assert_eq!(resp.collission, Some(false));
        assert_eq!(resp.hmds.expect("hmds").error.as_deref(), Some("no bridge"));
        let status = resp.iserver.and_then(|i| i.auth_status).expect("iserver.authStatus");
        assert_eq!(status.established, Some(true));
        assert!(resp.error.is_none());
    }

    #[test]
    fn decodes_official_fail_sample() {
        let resp: Response = serde_json::from_str(FAIL).expect("decode sample");
        assert_eq!(resp.error.as_deref(), Some("failed to process request"));
        assert!(resp.session.is_none());
    }
}
