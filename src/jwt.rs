use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, errors::Error as JwtError,
};
use serde::{Deserialize, Serialize};
use std::{
    sync::OnceLock,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (user_id)
    pub exp: u64,    // expiration time
}

const EXP_SECS: u64 = 60 * 60; // 1 hour

static SECRET_KEY: OnceLock<String> = OnceLock::new();

pub fn init(secret_key: String) {
    SECRET_KEY.set(secret_key).unwrap()
}

pub fn generate_jwt(sub: String) -> Result<String, JwtError> {
    let exp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + EXP_SECS;
    let claims = Claims { sub, exp };
    let header = Header::new(Algorithm::HS256);
    let secret_key: &String = SECRET_KEY.get().unwrap();
    let encoding_key = EncodingKey::from_secret(secret_key.as_bytes());

    jsonwebtoken::encode(&header, &claims, &encoding_key)
}

pub fn validate_jwt(token: &str) -> Result<Claims, JwtError> {
    let secret_key: &String = SECRET_KEY.get().unwrap();
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());

    let validation = Validation::new(Algorithm::HS256);

    jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}
