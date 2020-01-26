use crate::models::{User, NewUser};
use crate::schema::users;
use crate::{Repo, Connection};
use crate::auth::GoogleProfile;

use anyhow::{Result, Error};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use futures::Future;
use uuid::Uuid;

// use diesel::{RunQueryDsl, insert_into};
// use diesel::prelude::*;

// use uuid::Uuid;
//
// use crate::schema;
// use crate::models::{User, NewUser};
// use crate::api::{DBConnection};

pub fn insert(conn: &Connection, new_user: NewUser) -> Result<User>{
    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)?;

    Ok(result)
}

pub fn find_by_email(conn: &Connection, user_email: String) -> Result<Option<User>> {
    use crate::schema::users::dsl::*;

    let found_user = users
        .filter(email.eq(user_email))
        .first::<User>(conn);

    let result = match found_user {
        Ok(user) => Some(user),
        Err(_) => None
    };

    Ok(result)
}

pub fn find_or_create_by_profile(conn: &Connection, profile: GoogleProfile) -> Result<User> {
    let user = find_by_email(conn, profile.email.clone())?;

    match user {
        Some(user) => Ok(user),
        None => {
            let id = Uuid::new_v4();
            let new_user = NewUser {
                id,
                email: profile.email.clone(),
                name: profile.given_name.as_ref().unwrap_or(&String::from("")).to_string(),
                last_name: profile.family_name.clone(),
            };

            let user = insert(conn, new_user)?;
            Ok(user)
        }
    }
}

pub fn find_or_create_byProfile_repo(repo: Repo, profile: GoogleProfile)
    -> impl Future<Item = User, Error = Error>
{
    repo.run(|conn| {
        let user = find_or_create_by_profile(&conn, profile)?;

        Ok(user)
    })
}
