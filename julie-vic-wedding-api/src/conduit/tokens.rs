use crate::db::Repo;
use julie_vic_wedding_core::models::Token;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use futures::Future;

pub fn find(repo: Repo, token_str: String) -> impl Future<Item = Token, Error = DieselError> {
    use julie_vic_wedding_core::schema::tokens::dsl::*;

    repo.run(|conn| tokens.filter(token.eq(token_str)).first::<Token>(&conn))
}
