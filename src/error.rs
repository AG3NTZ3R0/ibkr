//! Crate-wide error type.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Transport failure: no response arrived.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    /// A non-2xx response. `body` is the response text, verbatim.
    #[error("api error {status}: {body}")]
    Api { status: u16, body: String },
    /// A 2xx response that did not match the expected shape. `body` is the
    /// response text, verbatim.
    #[error("decode error: {source}")]
    Decode { source: serde_json::Error, body: String },
    /// Local OAuth failure: key handling, signing, or token validation.
    #[error("auth error: {0}")]
    Auth(String),
}
