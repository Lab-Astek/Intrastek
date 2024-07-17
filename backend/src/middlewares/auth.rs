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
                debug!("Found matching key for kid: {}", kid);
                return DecodingKey::from_rsa_components(&key.n, &key.e).ok();
            }
        }
        warn!("No matching key found for kid: {}", kid);
        None
    }
}

async fn fetch_jwks() -> Result<Jwks, reqwest::Error> {
    let jwks_uri = "https://login.microsoftonline.com/common/discovery/v2.0/keys";

    let jwks: Jwks = reqwest::get(jwks_uri).await?.json::<Jwks>().await?;
    debug!("Fetched jwks: {:?}", jwks);
    Ok(jwks)
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
            Ok(c) => {
                info!("Token successfully decoded: {:?}", c.claims);
                Outcome::Success(AuthenticatedUser { claims: c.claims })
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
    validation.set_issuer(&["expected_issuer"]);
    validation.set_audience(&["expected_audience"]);
    validation
}
