//! `GET /iserver/marketdata/history` — historical market data for a contract.
//! Docs: <https://www.interactivebrokers.com/docs/web-api/api-reference/trading/trading-market-data/get-md-history>

use crate::Endpoint;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Request {
    pub conid: u64,
    pub bar: BarSize,
    pub period: Option<String>,
    pub exchange: Option<String>,
    pub start_time: Option<String>,
    pub outside_rth: Option<bool>,
    pub direction: Option<Direction>,
    pub source: Option<Source>,
}

/// Allowed `bar` values. The period-vs-bar "Step Size" matrix is not enforced here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarSize {
    Min1,
    Min2,
    Min3,
    Min5,
    Min10,
    Min15,
    Min30,
    Hour1,
    Hour2,
    Hour3,
    Hour4,
    Hour8,
    Day1,
    Week1,
    Month1,
}

impl BarSize {
    pub fn as_str(self) -> &'static str {
        match self {
            BarSize::Min1 => "1min",
            BarSize::Min2 => "2min",
            BarSize::Min3 => "3min",
            BarSize::Min5 => "5min",
            BarSize::Min10 => "10min",
            BarSize::Min15 => "15min",
            BarSize::Min30 => "30min",
            BarSize::Hour1 => "1h",
            BarSize::Hour2 => "2h",
            BarSize::Hour3 => "3h",
            BarSize::Hour4 => "4h",
            BarSize::Hour8 => "8h",
            BarSize::Day1 => "1d",
            BarSize::Week1 => "1w",
            BarSize::Month1 => "1m",
        }
    }
}

/// The reference contradicts itself on whether `Forward` requires `start_time`; neither rule
/// is enforced here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Backward,
    Forward,
}

impl Direction {
    pub fn as_str(self) -> &'static str {
        match self {
            Direction::Backward => "-1",
            Direction::Forward => "1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Last,
    Midpoint,
    BidAsk,
}

impl Source {
    pub fn as_str(self) -> &'static str {
        match self {
            Source::Last => "Last",
            Source::Midpoint => "Midpoint",
            Source::BidAsk => "Bid_Ask",
        }
    }
}

impl Endpoint for Request {
    type Response = Response;
    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn path(&self) -> String {
        "/iserver/marketdata/history".to_string()
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut q = vec![
            ("conid".to_string(), self.conid.to_string()),
            ("bar".to_string(), self.bar.as_str().to_string()),
        ];
        if let Some(period) = &self.period {
            q.push(("period".to_string(), period.clone()));
        }
        if let Some(exchange) = &self.exchange {
            q.push(("exchange".to_string(), exchange.clone()));
        }
        if let Some(start_time) = &self.start_time {
            q.push(("startTime".to_string(), start_time.clone()));
        }
        if let Some(outside_rth) = self.outside_rth {
            q.push(("outsideRth".to_string(), outside_rth.to_string()));
        }
        if let Some(direction) = self.direction {
            q.push(("direction".to_string(), direction.as_str().to_string()));
        }
        if let Some(source) = self.source {
            q.push(("source".to_string(), source.as_str().to_string()));
        }
        q
    }
}

/// `history-data` response. Fields are optional + `serde(default)` and unknown fields
/// ignored, so the docs' prose-vs-example drift never breaks the decode.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Response {
    pub server_id: Option<String>,
    pub symbol: Option<String>,
    pub text: Option<String>,
    pub price_factor: Option<i64>,
    pub start_time: Option<String>,
    /// Composite `%price/%volume/%minutes`, not a plain number.
    pub high: Option<String>,
    /// Formatted like [`Self::high`].
    pub low: Option<String>,
    pub time_period: Option<String>,
    pub bar_length: Option<i64>,
    pub md_availability: Option<String>,
    pub mkt_data_delay: Option<i64>,
    pub outside_rth: Option<bool>,
    pub trading_day_duration: Option<i64>,
    pub volume_factor: Option<i64>,
    pub price_display_rule: Option<i64>,
    pub price_display_value: Option<String>,
    pub chart_annotations: Option<String>,
    pub chart_pan_start_time: Option<String>,
    pub direction: Option<i64>,
    pub negative_capable: Option<bool>,
    pub message_version: Option<i64>,
    /// The historical bars.
    pub data: Vec<Bar>,
    /// Count of data points — not the bars themselves (those are [`Self::data`]).
    pub points: Option<i64>,
    pub travel_time: Option<i64>,
}

/// A single bar. Raw IBKR names; note the field order is o/c/h/l/v, not OHLC.
#[derive(Debug, Clone, Deserialize)]
pub struct Bar {
    /// Open.
    pub o: f64,
    /// Close.
    pub c: f64,
    /// High.
    pub h: f64,
    /// Low.
    pub l: f64,
    /// Volume.
    pub v: f64,
    /// Epoch unix timestamp.
    pub t: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Official example response from the docs (`Last` source).
    const SAMPLE: &str = r#"{
  "barLength": 86400,
  "chartPanStartTime": "20250521-00:00:00",
  "data": [
    {
      "c": 212.33,
      "h": 213.94,
      "l": 210.58,
      "o": 212.43,
      "t": 1747229400000,
      "v": 266616.18
    },
    {
      "c": 211.45,
      "h": 212.96,
      "l": 209.54,
      "o": 210.95,
      "t": 1747315800000,
      "v": 256847.25
    },
    {
      "c": 211.26,
      "h": 212.57,
      "l": 209.77,
      "o": 212.36,
      "t": 1747402200000,
      "v": 235240.24
    },
    {
      "c": 208.78,
      "h": 209.48,
      "l": 204.26,
      "o": 207.78,
      "t": 1747661400000,
      "v": 267569.89
    }
  ],
  "direction": -1,
  "high": "21394/266616.18/1440",
  "low": "20426/267569.89/8640",
  "mdAvailability": "S",
  "messageVersion": 2,
  "mktDataDelay": 0,
  "negativeCapable": false,
  "outsideRth": false,
  "points": 3,
  "priceDisplayRule": 1,
  "priceDisplayValue": "2",
  "priceFactor": 100,
  "serverId": "4155816",
  "startTime": "20250513-13:30:00",
  "symbol": "AAPL",
  "text": "APPLE INC",
  "timePeriod": "1w",
  "travelTime": 7,
  "volumeFactor": 100
}"#;

    #[test]
    fn decodes_official_sample() {
        let resp: Response = serde_json::from_str(SAMPLE).expect("decode sample");
        assert_eq!(resp.symbol.as_deref(), Some("AAPL"));
        assert_eq!(resp.price_factor, Some(100));
        assert_eq!(resp.high.as_deref(), Some("21394/266616.18/1440"));
        assert_eq!(resp.time_period.as_deref(), Some("1w"));
        assert_eq!(resp.direction, Some(-1));
        assert_eq!(resp.data.len(), 4);
        let bar = &resp.data[0];
        assert_eq!(bar.o, 212.43);
        assert_eq!(bar.c, 212.33);
        assert_eq!(bar.t, 1747229400000);
        assert_eq!(resp.points, Some(3));
    }

    #[test]
    fn ignores_unknown_fields() {
        let resp: Response =
            serde_json::from_str(r#"{"symbol":"AAPL","data":[],"futureField":123}"#)
                .expect("decode with unknown field");
        assert_eq!(resp.symbol.as_deref(), Some("AAPL"));
        assert!(resp.data.is_empty());
    }

    #[test]
    fn query_includes_required_and_set_optionals() {
        let req = Request {
            conid: 265598,
            bar: BarSize::Day1,
            period: Some("1w".to_string()),
            exchange: None,
            start_time: None,
            outside_rth: Some(true),
            direction: None,
            source: Some(Source::Last),
        };
        let q = req.query();
        assert!(q.contains(&("conid".to_string(), "265598".to_string())));
        assert!(q.contains(&("bar".to_string(), "1d".to_string())));
        assert!(q.contains(&("period".to_string(), "1w".to_string())));
        assert!(q.contains(&("outsideRth".to_string(), "true".to_string())));
        assert!(q.contains(&("source".to_string(), "Last".to_string())));
        // unset optionals are absent
        assert!(!q.iter().any(|(k, _)| k == "exchange"));
    }

    #[test]
    fn query_emits_direction_only_when_set() {
        let req = |direction| Request {
            conid: 265598,
            bar: BarSize::Day1,
            period: None,
            exchange: None,
            start_time: None,
            outside_rth: None,
            direction,
            source: None,
        };
        let dir = |q: Vec<(String, String)>| q.into_iter().find(|(k, _)| k == "direction").map(|(_, v)| v);
        assert_eq!(dir(req(Some(Direction::Forward)).query()).as_deref(), Some("1"));
        assert_eq!(dir(req(Some(Direction::Backward)).query()).as_deref(), Some("-1"));
        assert_eq!(dir(req(None).query()), None);
    }
}
