use gotham_middleware_diesel::{self};
use std::env;

// use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub type Repo = gotham_middleware_diesel::Repo<PgConnection>;
// pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn repo() -> Repo {
    let database_url =
    env::var("DATABASE_URL").unwrap_or("postgresql://postgres@localhost:5432".to_string());

    Repo::new(&database_url)
}
