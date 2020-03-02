use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;
use julie_vic_wedding_core::models::{NewToken, Token};
use julie_vic_wedding_core::schema::*;
use rand::Rng;
use uuid::Uuid;

pub fn generate_tokens(amount: u32, conn: PgConnection) {
    println!("Generate {} tokens", amount);

    for _ in 0..amount {
        let new_token = NewToken {
            id: Uuid::new_v4(),
            token: generate_random_token(),
        };

        let token = diesel::insert_into(tokens::table)
            .values(&new_token)
            .get_result::<Token>(&conn);

        match token {
            Ok(t) => {
                println!("Token: {}", t.token);
            }
            Err(e) => {
                println!("A token failed to generate: {:?}", e);
            }
        }
    }
}

pub fn generate_random_token() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const TOKEN_LEN: usize = 5;

    let mut rng = rand::thread_rng();

    let token: String = (0..TOKEN_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    token
}
