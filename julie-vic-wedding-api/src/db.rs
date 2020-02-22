use diesel::PgConnection;
use gotham_middleware_diesel::{self};
use std::env;

pub type Repo = gotham_middleware_diesel::Repo<PgConnection>;

pub fn repo() -> Repo {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres@localhost:5432".to_string());

    Repo::new(&database_url)
}
