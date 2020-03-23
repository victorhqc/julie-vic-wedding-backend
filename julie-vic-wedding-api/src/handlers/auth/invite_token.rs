use futures::{future, Future};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::create_empty_response;
use gotham::state::{FromState, State};
use hyper::StatusCode;

use crate::conduit::tokens;
use crate::handlers::extract_json;
use crate::Repo;

#[derive(Deserialize)]
struct VerifyRequest {
    token: String,
}

pub fn verify_invite_token(mut state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();

    let f = extract_json::<VerifyRequest>(&mut state)
        .and_then(move |body| {
            tokens::find(repo, body.token).map_err(|e| match e {
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
            match result {
                Err(e) => {
                    return future::err((state, e));
                }
                _ => {}
            };

            let res = create_empty_response(&state, StatusCode::OK);
            future::ok((state, res))
        });

    Box::new(f)
}
