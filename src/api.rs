use anyhow::{Result};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use std::env;

use crate::models::{User};
use crate::users::{create_user, find_by_email};
use crate::auth::{GoogleProfile};

pub struct Api {
    pool: r2d2::Pool<DBConnectionManager>,
}

impl Api {
    pub fn connect() -> Result<Self> {
        let database_url =
            env::var("DATABASE_URL").unwrap_or("postgresql://postgres@localhost:5432".to_string());
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::new(manager)?;

        // To get a connection simply do
        // let conn = pool.get()?;

        Ok(Self { pool })
    }

    pub fn get_connection(&self) -> Result<DBConnection> {
        let conn = self.pool.get()?;

        Ok(conn)
    }

    pub fn find_or_create_user(&self, profile: &GoogleProfile) -> Result<User> {
        let conn = self.get_connection()?;

        let user = find_by_email(&conn, profile.email.clone());

        match user {
            Some(user) => Ok(user),
            None => {
                let user = create_user(&conn, &profile)?;
                Ok(user)
            }
        }
    }
}

type DBConnectionManager = ConnectionManager<PgConnection>;
pub type DBConnection = PooledConnection<DBConnectionManager>;
