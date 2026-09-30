//! `POST /oauth/live_session_token` — the Diffie-Hellman handshake that mints a live session token.
//!
//! The only route that is not an [`Endpoint`](crate::Endpoint). It is a three-phase exchange, not a
//! single call: derive a DH challenge and RSA-sign the request, POST it, then complete the exchange
//! against the response to compute and validate the token. The middle phase can't be driven by
//! [`Client::send`](crate::Client::send) — the signing is RSA-SHA256 with the signature key rather
//! than HMAC keyed by the token, and the token being minted here is the very thing `send` needs to
//! sign anything. The phases share state (`a`, the decrypted secret), so they live together.
//!
//! Docs: <https://www.interactivebrokers.com/docs/web-api/api-reference/authentication/oauth-1-0-a/req-live-session-token>
//! Guide: <https://www.interactivebrokers.com/docs/web-api/authentication/oauth-1a/lst/compute-live-session-token>

use num_bigint::BigUint;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;

use crate::auth::{crypto, signed_header, Credentials, LiveSessionToken};
use crate::error::Error;

const PATH: &str = "/oauth/live_session_token";

/// Not public API: this route is not reachable via `send`, so nothing outside can use it.
#[derive(Debug, Clone, Deserialize)]
struct Response {
    /// Diffie-Hellman response `B`, hex.
    diffie_hellman_response: String,
    /// Proof used to validate the locally-computed token.
    live_session_token_signature: String,
    /// Epoch-millis expiration (~24h out).
    live_session_token_expiration: i64,
}

/// Run the exchange against `base` and return the validated token.
pub fn mint(
    creds: &Credentials,
    http: &reqwest::blocking::Client,
    base: &str,
) -> Result<LiveSessionToken, Error> {
    let (prime, generator) = crypto::parse_dh_params(&creds.dhparam_pem)?;
    let a = crypto::dh_random();
    let challenge = generator.modpow(&a, &prime).to_str_radix(16);

    // Decrypt the access token secret; its hex prepends the base string, its bytes key the LST.
    let enc_key = crypto::load_private_key(&creds.private_encryption_pem)?;
    let secret = crypto::rsa_decrypt(&enc_key, &crypto::b64_decode(&creds.access_token_secret)?)?;
    let prepend = crypto::hex_encode(&secret);

    let url = format!("{base}{PATH}");
    let params = vec![
        ("diffie_hellman_challenge".into(), challenge),
        ("oauth_consumer_key".into(), creds.consumer_key.clone()),
        ("oauth_nonce".into(), crypto::nonce()),
        ("oauth_signature_method".into(), "RSA-SHA256".into()),
        ("oauth_timestamp".into(), crypto::timestamp()),
        ("oauth_token".into(), creds.access_token.clone()),
    ];

    let base_string = format!("{prepend}{}", crypto::signature_base_string("POST", &url, &params));
    let sig_key = crypto::load_private_key(&creds.private_signature_pem)?;
    let signature =
        crypto::percent_encode(&crypto::b64_encode(&crypto::rsa_sha256_sign(&sig_key, base_string.as_bytes())?));

    let http_resp = http
        .post(&url)
        .header(AUTHORIZATION, signed_header(params, signature, &creds.realm))
        .body("") // gateway 411s an empty POST without an explicit Content-Length: 0
        .send()?;
    let status = http_resp.status();
    let text = http_resp.text()?;
    if !status.is_success() {
        return Err(Error::Api { status: status.as_u16(), body: text });
    }
    let resp: Response =
        serde_json::from_str(&text).map_err(|source| Error::Decode { source, body: text })?;

    // K = B^a mod p; LST = HMAC_SHA1(K_bytes, secret_bytes).
    let b = BigUint::parse_bytes(resp.diffie_hellman_response.as_bytes(), 16)
        .ok_or_else(|| Error::Auth("diffie_hellman_response is not valid hex".into()))?;
    let k = b.modpow(&a, &prime);
    let lst = crypto::hmac_sha1(&k.to_bytes_be(), &secret);

    // Validate against the signature the gateway returned.
    let check = crypto::hex_encode(&crypto::hmac_sha1(&lst, creds.consumer_key.as_bytes()));
    if check != resp.live_session_token_signature {
        return Err(Error::Auth("live session token validation failed".into()));
    }

    Ok(LiveSessionToken {
        token: crypto::b64_encode(&lst),
        expiration: resp.live_session_token_expiration,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Official example response from the docs.
    const SAMPLE: &str = r#"{
  "live_session_token_signature": "712549e22ba937ab10bc8d571bd3a9c20c43d7cb",
  "live_session_token_expiration": 1714669716258,
  "diffie_hellman_response": "74d222b7c3c0916eb3c7e60ecec937cd4f531aab5ea5698ebd754feeb39a9d7fe04c2887b5e11d8a8e4176c2eee7ddd42016e57c7a98cfbb9a3282c5247f7d4a9b8f544a34fac4d6334065b55d6a2e73e390175cfb94e80281443e555030c576d1db409bf96870dab20581bc01cdf28489778f20c714e7ad39af60c476cf2207a119df3af82bbefcdad7749fa5b4ae6e93169ec14f66ff3220cf3156487ca33932284b0a09af14f05a75269ab243362ff4eabb2e2a57db0d7911ca549f24affa1f92f04908fd9a2349cefb0f9326aca65fc144847ab837fcbd1635b1aa84b4509198e349fad87c2caf6744cb94be2c5c1c7ef9f08c44e85ded45ebeefa248dc8"
}"#;

    #[test]
    fn decodes_official_sample() {
        let resp: Response = serde_json::from_str(SAMPLE).expect("decode sample");
        assert!(resp.diffie_hellman_response.starts_with("74d222b7"));
        assert_eq!(resp.live_session_token_signature, "712549e22ba937ab10bc8d571bd3a9c20c43d7cb");
        assert_eq!(resp.live_session_token_expiration, 1714669716258);
    }
}
