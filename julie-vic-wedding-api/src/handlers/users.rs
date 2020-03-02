use futures::{future, Future};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::{FromState, State};
use gotham_middleware_jwt::AuthorizationToken;
use hyper::StatusCode;

use crate::auth::AuthUser;
use crate::conduit::users;
use crate::handlers::{extract_json, wrap_error};
use crate::Repo;
use julie_vic_wedding_core::attend_status_type::AttendStatus;
use julie_vic_wedding_core::models::{ConfirmedUser, User};

#[derive(Serialize)]
pub struct UserResponse {
    user: User,
}

pub fn me(state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let token = AuthorizationToken::<AuthUser>::borrow_from(&state);
    let email = token.0.claims.email();

    let results = users::find_by_email(repo, email).then(|result| match result {
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

#[derive(Deserialize)]
pub struct RsvpRequest {
    will_attend: bool,
    plus_one: bool,
    token: String,
}

#[derive(Serialize)]
pub struct RsvpResponse {
    confirmed_user: ConfirmedUser,
}

pub fn rsvp(mut state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let token = AuthorizationToken::<AuthUser>::borrow_from(&state);
    let user_id = token.0.claims.user_id();

    let f = extract_json::<RsvpRequest>(&mut state)
        .and_then(move |body| {
            let user_data = users::NewUserData {
                user_id,
                will_attend: get_attend_status(&body),
                table_id: None,
                token: body.token,
            };

            users::rsvp_confirmation(repo, user_data).map_err(|e| match e {
                diesel::result::Error::DatabaseError(_, _) => e
                    .into_handler_error()
                    .with_status(StatusCode::INTERNAL_SERVER_ERROR),
                diesel::result::Error::NotFound => {
                    e.into_handler_error().with_status(StatusCode::UNAUTHORIZED)
                }
                _ => e.into_handler_error().with_status(StatusCode::BAD_REQUEST),
            })
        })
        .then(|result| {
            let confirmed_user = match result {
                Ok(u) => u,
                Err(e) => {
                    return future::err((state, e));
                }
            };

            let body = match serde_json::to_string(&RsvpResponse { confirmed_user }) {
                Ok(b) => b,
                Err(e) => {
                    let f = wrap_error(state, e, StatusCode::INTERNAL_SERVER_ERROR);
                    return future::err(f);
                }
            };

            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
            future::ok((state, res))
        });

    Box::new(f)
}

fn get_attend_status(req: &RsvpRequest) -> AttendStatus {
    match (req.will_attend, req.plus_one) {
        (false, _) => AttendStatus::No,
        (true, false) => AttendStatus::Yes,
        (true, true) => AttendStatus::YesPlusOne,
    }
}
