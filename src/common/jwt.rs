use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use time::Duration;

const SECRET: &'static str = "jwt";

pub fn get_token(id: u64) -> String {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::hours(2)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: id.to_string(),
        exp,
        iat,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[test]
fn get_jwt_token() {
    let token = get_token(12);
    println!("{}", token)
}
