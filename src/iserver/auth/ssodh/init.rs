//! `POST /iserver/auth/ssodh/init` — open (or re-establish) the brokerage session after auth.
//! `publish`/`compete` travel as a JSON body (not signed), matching IBKR's reference client.
//! Docs: <https://www.interactivebrokers.com/docs/web-api/api-reference/trading/trading-session/initialize-session>

use crate::Endpoint;
use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct Request {
    pub publish: bool,
    pub compete: bool,
}

impl Endpoint for Request {
    type Response = Response;
    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn path(&self) -> String {
        "/iserver/auth/ssodh/init".to_string()
    }

    fn body(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({ "publish": self.publish, "compete": self.compete }))
    }
}

/// Fields optional + `serde(default)`, unknown ignored.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Response {
    pub authenticated: Option<bool>,
    pub competing: Option<bool>,
    pub connected: Option<bool>,
    pub message: Option<String>,
    pub fail: Option<String>,
    pub established: Option<bool>,
    pub server_info: Option<ServerInfo>,
    // `MAC` and `hardware_info` fall outside the camelCase rename on the wire.
    #[serde(rename = "MAC")]
    pub mac: Option<String>,
    #[serde(rename = "hardware_info")]
    pub hardware_info: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ServerInfo {
    pub server_name: Option<String>,
    pub server_version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Official example response from the docs.
    const SAMPLE: &str = r#"{
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
}"#;

    #[test]
    fn decodes_official_sample() {
        let resp: Response = serde_json::from_str(SAMPLE).expect("decode sample");
        assert_eq!(resp.authenticated, Some(true));
        assert_eq!(resp.established, Some(true));
        assert_eq!(resp.mac.as_deref(), Some("98:F2:B3:23:BF:A0"));
        let info = resp.server_info.expect("serverInfo");
        assert_eq!(info.server_name.as_deref(), Some("JifN19053"));
        assert_eq!(info.server_version.as_deref(), Some("Build 10.25.0p, Dec 5, 2023 5:48:12 PM"));
    }
}
