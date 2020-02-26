use futures::{future, Future};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::{create_response, create_temporary_redirect};
use gotham::state::{FromState, State};
use hyper::StatusCode;
use julie_vic_wedding_core::models::User;
use std::env;

use crate::auth::encode_token;
use crate::auth::facebook::{
    build_client, exchange_token, gen_authorize_url, get_user_profile, FacebookRedirectExtractor,
};
use crate::conduit::users::find_or_create;
use crate::handlers::error_handler;
use crate::Repo;

#[derive(Serialize)]
struct AuthenticatedUser {
    user: User,
    token: String,
}

pub fn facebook_authorize_handler(state: State) -> Box<HandlerFuture> {
    let facebook_client = match build_client() {
        Ok(c) => c,
        Err(e) => return error_handler(state, e),
    };

    let (authorize_url, _) = gen_authorize_url(facebook_client);
    let res = create_temporary_redirect(&state, authorize_url.to_string());
    let f = future::ok((state, res));

    Box::new(f)
}

pub fn facebook_redirect_handler(mut state: State) -> Box<HandlerFuture> {
    let query_param = FacebookRedirectExtractor::take_from(&mut state);
    let facebook_client = match build_client() {
        Ok(c) => c,
        Err(e) => return error_handler(state, e),
    };

    let token = match exchange_token(&query_param, &facebook_client) {
        Ok(t) => t,
        Err(e) => return error_handler(state, e),
    };

    let profile = match get_user_profile(&token) {
        Ok(p) => p,
        Err(e) => return error_handler(state, e),
    };

    let repo = Repo::borrow_from(&state).clone();
    let results = find_or_create(repo, profile).then(|result| match result {
        Ok(user) => {
            let token = encode_token(&user, 3600);
            let redirect_url = env::var("REDIRECT_CLIENT_URL");

            let res = match redirect_url {
                Ok(u) => create_temporary_redirect(&state, format!("{}?token={}", u, token)),
                _ => {
                    let response = AuthenticatedUser { user, token };
                    let body = serde_json::to_string(&response).expect("Failed to serialize user.");
                    create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body)
                }
            };

            future::ok((state, res))
        }
        Err(e) => {
            let err = e
                .into_handler_error()
                .with_status(StatusCode::INTERNAL_SERVER_ERROR);
            future::err((state, err))
        }
    });

    Box::new(results)
}
