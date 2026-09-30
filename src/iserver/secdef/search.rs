//! `GET /iserver/secdef/search` — search contracts by symbol or company name.
//! Docs: <https://www.interactivebrokers.com/docs/web-api/api-reference/trading/trading-contracts/get-contract-symbols>

use crate::Endpoint;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Request {
    pub symbol: String,
    pub name: Option<bool>,
    pub sec_type: Option<SecType>,
    pub more: Option<bool>,
    pub fund: Option<bool>,
    pub fund_family_conid_ex: Option<String>,
    pub pattern: Option<bool>,
    pub referrer: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecType {
    Stk,
    Ind,
    Bond,
}

impl SecType {
    pub fn as_str(self) -> &'static str {
        match self {
            SecType::Stk => "STK",
            SecType::Ind => "IND",
            SecType::Bond => "BOND",
        }
    }
}

impl Endpoint for Request {
    type Response = Vec<Contract>;
    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn path(&self) -> String {
        "/iserver/secdef/search".to_string()
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut q = vec![("symbol".to_string(), self.symbol.clone())];
        if let Some(name) = self.name {
            q.push(("name".to_string(), name.to_string()));
        }
        if let Some(sec_type) = self.sec_type {
            q.push(("secType".to_string(), sec_type.as_str().to_string()));
        }
        if let Some(more) = self.more {
            q.push(("more".to_string(), more.to_string()));
        }
        if let Some(fund) = self.fund {
            q.push(("fund".to_string(), fund.to_string()));
        }
        if let Some(fund_family_conid_ex) = &self.fund_family_conid_ex {
            q.push(("fundFamilyConidEx".to_string(), fund_family_conid_ex.clone()));
        }
        if let Some(pattern) = self.pattern {
            q.push(("pattern".to_string(), pattern.to_string()));
        }
        if let Some(referrer) = &self.referrer {
            q.push(("referrer".to_string(), referrer.clone()));
        }
        q
    }
}

/// One matched contract. Fields are optional + `serde(default)` and unknown fields
/// ignored; bond results populate [`Self::issuers`]/[`Self::bond_id`] and leave the
/// company fields null.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Contract {
    pub conid: Option<String>,
    pub company_header: Option<String>,
    pub company_name: Option<String>,
    pub symbol: Option<String>,
    pub description: Option<String>,
    /// Comma-separated secTypes barred from trading, e.g. `CFD,IOPT` — not a bool.
    pub restricted: Option<String>,
    pub sec_type: Option<String>,
    pub sections: Vec<Section>,
    pub issuers: Vec<Issuer>,
    /// Wire key is lowercase `bondid`, outside the crate-wide camelCase rename.
    #[serde(rename = "bondid")]
    pub bond_id: Option<i64>,
    pub fop: Option<String>,
    pub opt: Option<String>,
    pub war: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Section {
    pub sec_type: Option<String>,
    /// Semicolon-separated, e.g. `JANYY;FEBYY;MARYY`.
    pub months: Option<String>,
    pub symbol: Option<String>,
    /// Semicolon-separated, e.g. `EXCH;EXCH;EXCH`.
    pub exchange: Option<String>,
    pub conid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Issuer {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Official example response from the docs.
    const SAMPLE: &str = r#"[
  {
    "conid": "8314",
    "companyHeader": "INTL BUSINESS MACHINES CORP - NYSE",
    "companyName": "INTL BUSINESS MACHINES CORP",
    "symbol": "IBM",
    "description": "NYSE",
    "restricted": null,
    "fop": null,
    "opt": "20240315;20240322;20240328;20240405;20240412;20240419;20240426;20240517;20240621;20240719;20240920;20241018;20250117;20250620;20260116",
    "war": "20240208;20240213;20240214;20240215;20240216;20240222;20240226;20240227;20240228;20240229;20240301;20240304;20240305;20240306;20240307;20240308;20240311;20240312;20240313;20240314;20240315;20240416;20240418;20240514;20240516;20240618;20240619;20240620;20240621;20240718;20240917;20240918;20240919;20240920;20241017;20241217;20241218;20241219;20241220;20250114;20250115;20250116;20250117;20250319;20250320;20250617;20250618;20250619;20250620;20250918;20251216;20251218;20260113;20260115",
    "sections": [
      {
        "secType": "STK"
      },
      {
        "secType": "OPT",
        "months": "MAR24;APR24;MAY24;JUN24;JUL24;SEP24;OCT24;JAN25;JUN25;JAN26",
        "exchange": "SMART;AMEX;BATS;BOX;CBOE;CBOE2;EDGX;EMERALD;GEMINI;IBUSOPT;ISE;MEMX;MERCURY;MIAX;NASDAQBX;NASDAQOM;PEARL;PHLX;PSE"
      },
      {
        "secType": "WAR",
        "months": "FEB24;MAR24;APR24;MAY24;JUN24;JUL24;SEP24;OCT24;DEC24;JAN25;MAR25;JUN25;SEP25;DEC25;JAN26",
        "exchange": "EBS;FWB;GETTEX;SBF;SWB"
      },
      {
        "secType": "IOPT"
      },
      {
        "secType": "CFD",
        "exchange": "SMART",
        "conid": "118239202"
      },
      {
        "secType": "BAG"
      }
    ]
  },
  {
    "conid": "41645598",
    "companyHeader": "INTL BUSINESS MACHINES CORP - LSE",
    "companyName": "INTL BUSINESS MACHINES CORP",
    "symbol": "IBM",
    "description": "LSE",
    "restricted": null,
    "fop": null,
    "opt": null,
    "war": null,
    "sections": [
      {
        "secType": "STK"
      }
    ]
  },
  {
    "bondid": 5,
    "conid": "2147483647",
    "companyHeader": "Corporate Fixed Income",
    "companyName": null,
    "restricted": null,
    "fop": null,
    "opt": null,
    "war": null,
    "sections": [
      {
        "secType": "BOND"
      }
    ],
    "issuers": [
      {
        "id": "e5499005",
        "name": "IBM International Capital Pte Ltd"
      },
      {
        "id": "e1580374",
        "name": "IBM-CALL"
      },
      {
        "id": "e1400789",
        "name": "International Business Machines Corp"
      },
      {
        "id": "e1658116",
        "name": "Truven Health Analytics Inc"
      }
    ]
  }
]"#;

    #[test]
    fn decodes_official_sample() {
        let resp: Vec<Contract> = serde_json::from_str(SAMPLE).expect("decode sample");
        assert_eq!(resp.len(), 3);
        let c = &resp[0];
        assert_eq!(c.conid.as_deref(), Some("8314"));
        assert_eq!(c.symbol.as_deref(), Some("IBM"));
        assert_eq!(c.fop, None);
        assert!(c.opt.as_deref().is_some_and(|d| d.starts_with("20240315;")));
        assert!(c.war.as_deref().is_some_and(|d| d.starts_with("20240208;")));
        assert_eq!(c.sections.len(), 6);
        let cfd = c.sections.iter().find(|s| s.sec_type.as_deref() == Some("CFD")).unwrap();
        assert_eq!(cfd.conid.as_deref(), Some("118239202"));

        let bond = &resp[2];
        assert_eq!(bond.bond_id, Some(5));
        assert_eq!(bond.issuers.len(), 4);
        assert_eq!(bond.issuers[0].id.as_deref(), Some("e5499005"));
    }

    #[test]
    fn ignores_unknown_fields() {
        let resp: Vec<Contract> =
            serde_json::from_str(r#"[{"conid":"265598","symbol":"AAPL","futureField":123}]"#)
                .expect("decode with unknown field");
        assert_eq!(resp[0].conid.as_deref(), Some("265598"));
        assert_eq!(resp[0].symbol.as_deref(), Some("AAPL"));
    }

    #[test]
    fn query_includes_required_and_set_optionals() {
        let req = Request {
            symbol: "IBKR".to_string(),
            name: Some(true),
            sec_type: Some(SecType::Stk),
            more: Some(true),
            fund: Some(false),
            fund_family_conid_ex: Some("123".to_string()),
            pattern: Some(true),
            referrer: Some("onl".to_string()),
        };
        let q = req.query();
        assert!(q.contains(&("symbol".to_string(), "IBKR".to_string())));
        assert!(q.contains(&("name".to_string(), "true".to_string())));
        assert!(q.contains(&("secType".to_string(), "STK".to_string())));
        assert!(q.contains(&("more".to_string(), "true".to_string())));
        assert!(q.contains(&("fund".to_string(), "false".to_string())));
        assert!(q.contains(&("fundFamilyConidEx".to_string(), "123".to_string())));
        assert!(q.contains(&("pattern".to_string(), "true".to_string())));
        assert!(q.contains(&("referrer".to_string(), "onl".to_string())));

        // unset optionals are absent
        let bare = Request {
            symbol: "AAPL".to_string(),
            name: None,
            sec_type: None,
            more: None,
            fund: None,
            fund_family_conid_ex: None,
            pattern: None,
            referrer: None,
        };
        let q = bare.query();
        assert_eq!(q.len(), 1);
        for key in ["name", "secType", "more", "fund", "fundFamilyConidEx", "pattern", "referrer"] {
            assert!(!q.iter().any(|(k, _)| k == key), "{key} should be absent");
        }
    }
}
