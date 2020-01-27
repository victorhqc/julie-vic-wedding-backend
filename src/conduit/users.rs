use crate::auth::GoogleProfile;
use crate::models::{NewUser, User, NewConfirmedUser, ConfirmedUser};
use crate::schema::{users, confirmed_users};

use crate::Repo;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use futures::Future;
use uuid::Uuid;

// pub fn insert(repo: Repo, new_user: NewUser) -> impl Future<Item = User, Error = DieselError> {
//     repo.run(move |conn| {
//         diesel::insert_into(users::table)
//             .values(&new_user)
//             .get_result(&conn)
//     })
// }

pub fn find_by_email(
    repo: Repo,
    user_email: String,
) -> impl Future<Item = User, Error = DieselError> {
    use crate::schema::users::dsl::*;
    repo.run(|conn| users.filter(email.eq(user_email)).first::<User>(&conn))
}

// TODO: Reduce code repetition here. I have no idea to implement an elegant solution right now.
// Maybe when moving to "async" is easy...
pub fn find_or_create(
    repo: Repo,
    profile: GoogleProfile,
) -> impl Future<Item = User, Error = DieselError> {
    repo.run(move |conn| {
        let user = {
            use crate::schema::users::dsl::*;

            users
                .filter(email.eq(profile.email.clone()))
                .first::<User>(&conn)
        };

        match user {
            Ok(u) => Ok(u),
            Err(_e) => {
                let id = Uuid::new_v4();
                let new_user = NewUser {
                    id,
                    email: profile.email.clone(),
                    name: profile
                        .given_name
                        .as_ref()
                        .unwrap_or(&String::from(""))
                        .to_string(),
                    last_name: profile.family_name.clone(),
                };

                diesel::insert_into(users::table)
                    .values(&new_user)
                    .get_result(&conn)
            }
        }
    })
}

pub fn rsvp_confirmation(repo: Repo, confirmed_user: NewConfirmedUser)
    -> impl Future<Item = ConfirmedUser, Error = DieselError>
{
    repo.run(move |conn| {
        diesel::insert_into(confirmed_users::table)
            .values(&confirmed_user)
            .get_result(&conn)
    })
}
