use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use log::{error, info, warn};

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
    // #[serde(default)]
    alg: Option<String>,
}

pub struct KeyStore {
    pub keys: RwLock<Jwks>,
}

impl KeyStore {
    pub async fn new() -> Self {
        warn!("Creating a new KeyStore...");
        let jwks = fetch_jwks().await.unwrap();
        KeyStore {
            keys: RwLock::new(jwks),
        }
    }

    pub async fn get_decoding_key(&self, kid: &str) -> Option<DecodingKey> {
        let keys = self.keys.read().await;
        for key in &keys.keys {
            if key.kid == kid {
                return Some(DecodingKey::from_rsa_components(&key.n, &key.e).unwrap());
            }
        }
        None
    }
}

async fn fetch_jwks() -> Result<Jwks, reqwest::Error> {
    let jwks_uri = "https://login.microsoftonline.com/common/discovery/keys";
    let jwks: Jwks = reqwest::get(jwks_uri).await?.json::<Jwks>().await?;
    warn!("Fetched jwks: {:?}", jwks);
    Ok(jwks)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        info!("Authenticating from request...");
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            warn!("Authorization header missing or has multiple values");
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token = keys[0].trim_start_matches("Bearer ");
        info!("Token received: {}", token);

        let key_store = request.rocket().state::<Arc<KeyStore>>().unwrap();

        let header = decode_header(token).unwrap();
        info!("Token header: {:?}", header);

        let kid = match header.kid {
            Some(kid) => {
                info!("Token kid: {}", kid);
                kid
            }
            None => {
                warn!("Token kid is missing");
                return Outcome::Error((Status::Unauthorized, ()));
            }
        };

        let decoding_key = match key_store.get_decoding_key(&kid).await {
            Some(key) => {
                info!("Using decoding key for kid: {}", kid);
                key
            }
            None => {
                warn!("No decoding key found for kid: {}", kid);
                return Outcome::Error((Status::Unauthorized, ()));
            }
        };

        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;
        validation.set_issuer(&["expected_issuer"]);
        validation.set_audience(&["expected_audience"]);

        match decode::<Claims>(token, &decoding_key, &validation) {
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
