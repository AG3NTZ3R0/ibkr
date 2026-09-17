//! Crate-wide error type.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Transport failure: no response arrived.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    /// A non-2xx response. `body` is the response text, verbatim.
    #[error("api error {status}: {body}")]
    Api { status: u16, body: String },
    #[error("decode error: {0}")]
    Decode(#[from] serde_json::Error),
    /// Local OAuth failure: key handling, signing, or token validation.
    #[error("auth error: {0}")]
    Auth(String),
}
