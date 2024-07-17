use jsonwebtoken::{
    decode, decode_header, errors::Error as JwtError, errors::ErrorKind as JwtErrorKind, Algorithm,
    DecodingKey, Validation,
};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use log::{debug, error, info, warn};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    aud: String,
    exp: usize,
    iss: String,
    sub: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthenticatedUser {
    pub claims: Claims,
}

#[derive(Debug, Deserialize)]
pub struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
pub struct Jwk {
    kid: String,
    n: String,
    e: String,
    kty: String,
    alg: Option<String>,
}

pub struct KeyStore {
    pub keys: RwLock<Jwks>,
}

impl KeyStore {
    pub async fn new() -> Self {
        warn!("Creating a new KeyStore...");

        let jwks = fetch_jwks().await.expect("Failed to fetch JWKS");
        KeyStore {
            keys: RwLock::new(jwks),
        }
    }

    pub async fn get_decoding_key(&self, kid: &str) -> Option<DecodingKey> {
        let keys = self.keys.read().await;
        for key in &keys.keys {
            if key.kid == kid {
                debug!("Found matching key for kid:\n\t{}={}", kid, key.kid);
                let alg = key.alg.clone().unwrap_or_else(|| {
                    if key.kty == "RSA" {
                        "RS256".to_string()
                    } else {
                        "none".to_string()
                    }
                });
                if alg == "RS256" && key.kty == "RSA" {
                    return DecodingKey::from_rsa_components(&key.n, &key.e).ok();
                } else {
                    warn!("Unsupported algorithm: {} for kid: {}", alg, kid);
                    return None;
                }
            }
        }
        warn!("No matching key found for kid: {}", kid);
        None
    }
}

async fn fetch_jwks() -> Result<Jwks, reqwest::Error> {
    let jwks_uri = "https://login.microsoftonline.com/common/discovery/v2.0/keys";

    let response = reqwest::get(jwks_uri).await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let jwks = resp.json::<Jwks>().await;
                match jwks {
                    Ok(jwks) => {
                        debug!("Fetched and parsed jwks: {:?}", jwks);
                        Ok(jwks)
                    }
                    Err(err) => {
                        error!("Error parsing JWKS response: {:?}", err);
                        Err(err)
                    }
                }
            } else {
                error!("Failed to fetch JWKS, status: {:?}", resp.status());
                Err(resp.error_for_status().unwrap_err())
            }
        }
        Err(err) => {
            error!("Network error fetching JWKS: {:?}", err);
            Err(err)
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        info!("Authenticating from request...");

        let token = match get_bearer_token(request) {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        info!("Token received: {}", token);

        let key_store = match request.rocket().state::<Arc<KeyStore>>() {
            Some(store) => store,
            None => {
                error!("KeyStore state is not initialized");
                return Outcome::Error((Status::InternalServerError, ()));
            }
        };

        let kid = match extract_kid(&token) {
            Ok(kid) => kid,
            Err(err) => {
                error!("Error extracting kid: {}", err);
                return Outcome::Error((Status::Unauthorized, ()));
            }
        };
        info!("Token kid: {}", kid);

        let decoding_key = match key_store.get_decoding_key(&kid).await {
            Some(key) => {
                debug!("Using decoding key for kid: {}", kid);
                key
            }
            None => {
                warn!("No decoding key found for kid: {}", kid);
                return Outcome::Error((Status::Unauthorized, ()));
            }
        };

        let validation = get_validation();
        match decode::<Claims>(&token, &decoding_key, &validation) {
            Ok(token_data) => {
                debug!("claims.iss: {:?}", token_data.claims.iss);
                debug!("claims.aud: {:?}", token_data.claims.aud);
                debug!("claims.exp: {:?}", token_data.claims.exp);

                if token_data.claims.iss != "https://login.microsoftonline.com/{tenantid}/v2.0" {
                    error!("Invalid issuer claim");
                    return Outcome::Error((Status::Unauthorized, ()));
                }

                if token_data.claims.aud != "expected_audience" {
                    error!("Invalid audience claim");
                    return Outcome::Error((Status::Unauthorized, ()));
                }

                let current_time: i64 = chrono::Utc::now().timestamp();
                if (token_data.claims.exp as i64) < current_time {
                    error!("Token has expired");
                    return Outcome::Error((Status::Unauthorized, ()));
                }

                info!("Token successfully decoded: {:?}", token_data.claims);
                Outcome::Success(AuthenticatedUser {
                    claims: token_data.claims,
                })
            }
            Err(err) => {
                error!("Error decoding token: {:?}", err);
                Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}

fn get_bearer_token(request: &Request<'_>) -> Option<String> {
    request
        .headers()
        .get_one("Authorization")
        .and_then(|bearer| {
            if bearer.starts_with("Bearer ") {
                Some(bearer.trim_start_matches("Bearer ").to_string())
            } else {
                warn!("Authorization header missing or improperly formatted");
                None
            }
        })
}

fn extract_kid(token: &str) -> Result<String, JwtError> {
    let header = decode_header(token)?;
    header.kid.ok_or_else(|| {
        JwtError::from(JwtErrorKind::MissingRequiredClaim(String::from(
            "Kid is missing",
        )))
    })
}

fn get_validation() -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    validation.set_issuer(&[
        "https://login.microsoftonline.com/901cb4ca-b862-4029-9306-e5cd0f6d9f86/v2.0",
    ]);
    validation.set_audience(&["b5c2e510-4a17-4feb-b219-e55aa5b74144"]);
    debug!("validation {:?}", validation);
    validation
}
