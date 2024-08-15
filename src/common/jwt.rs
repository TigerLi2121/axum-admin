use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET: &'static str = "jwt";

pub fn get_token(id: u64) -> String {
    let now = Local::now();
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

pub fn get_uid(token: String) -> u64 {
    match decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::default(),
    ) {
        Ok(c) => c.claims.sub.parse().unwrap(),
        Err(_) => 0,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[test]
fn jwt_token() {
    let token = get_token(11);
    println!("token {}", token);
    let uid = get_uid(token);
    println!("uid {}", uid)
}
