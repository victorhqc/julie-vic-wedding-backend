use diesel::connection::Connection;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres@localhost:5432".to_string());
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
