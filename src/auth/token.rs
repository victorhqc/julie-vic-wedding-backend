use jsonwebtoken::{encode, Header};
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthUser {
    email: String,
    exp: u64,
}

impl AuthUser {
    pub fn new(email: String, expire_in: u64) -> Self {
        AuthUser {
            email: email.clone(),
            exp: seconds_from_now(expire_in),
        }
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }
}

pub fn get_secret() -> String {
    env::var("TOKEN_SECRET").expect("TOKEN_SECRET variable is not defined")
}

pub fn encode_token(email: String, expire_in: u64) -> String {
    let secret = get_secret();

    encode(
        &Header::default(),
        &AuthUser::new(email, expire_in),
        secret.as_ref(),
    )
    .unwrap()
}

fn seconds_from_now(secs: u64) -> u64 {
    let expiry_time =
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + Duration::from_secs(secs);
    expiry_time.as_secs()
}
