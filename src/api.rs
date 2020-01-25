use anyhow::Result;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use std::env;
use std::sync::{Arc, Mutex};

use crate::auth::GoogleProfile;
use crate::models::User;
use crate::users::{create_user, find_by_email};

#[derive(Clone, StateData)]
pub struct Api {
    pool: Arc<Mutex<r2d2::Pool<DBConnectionManager>>>,
}

impl Api {
    pub fn connect() -> Self {
        let database_url =
            env::var("DATABASE_URL").unwrap_or("postgresql://postgres@localhost:5432".to_string());
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::new(manager).expect("Failed to connect DB Pool");

        // To get a connection simply do
        // let conn = pool.get()?;

        Self {
            pool: Arc::new(Mutex::new(pool)),
        }
    }

    pub fn get_connection(&self) -> Result<DBConnection> {
        let conn = self.pool.lock().unwrap().get()?;

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
