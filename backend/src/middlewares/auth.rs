use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use log::{info, warn};

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
    alg: String,
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
    warn!("Fetched jwks!");
    Ok(jwks)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        info!("Authenticating from request...");
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token = keys[0].trim_start_matches("Bearer ");

        let key_store = request.rocket().state::<Arc<KeyStore>>().unwrap();

        let header = decode_header(token).unwrap();
        let kid = header.kid.unwrap();

        let decoding_key = match key_store.get_decoding_key(&kid).await {
            Some(key) => key,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        // let validation = Validation {
        //     algorithms: vec![Algorithm::RS256],
        //     ..Validation::default()
        // };

        match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)) {
            Ok(c) => Outcome::Success(AuthenticatedUser { claims: c.claims }),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
