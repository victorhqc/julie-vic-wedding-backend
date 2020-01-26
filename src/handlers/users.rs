use futures::{future, Future};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::{FromState, State};
use gotham_middleware_jwt::AuthorizationToken;
use hyper::StatusCode;

use crate::auth::AuthUser;
use crate::conduit::users;
use crate::models::User;
use crate::Repo;

#[derive(Serialize)]
pub struct UserResponse {
    user: User,
}

pub fn me(state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let token = AuthorizationToken::<AuthUser>::borrow_from(&state);
    let email = token.0.claims.email();
    println!("TOKEN {}", token.0.claims.email());

    let results = users::find_by_email(repo.clone(), email).then(|result| match result {
        Ok(user) => {
            let response = UserResponse { user };
            let body = serde_json::to_string(&response).expect("Failed to serialize user.");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
            future::ok((state, res))
        }
        Err(diesel::result::Error::NotFound) => {
            let res = create_empty_response(&state, StatusCode::UNAUTHORIZED);
            future::ok((state, res))
        }
        Err(e) => future::err((state, e.into_handler_error())),
    });

    Box::new(results)
}
