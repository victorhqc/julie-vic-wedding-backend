use futures::{future, Future};
use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::{
    create_empty_response, create_response, create_temporary_redirect,
};
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};
use julie_vic_wedding_core::models::User;
use std::env;

use crate::auth::encode_token;
use crate::auth::google::{
    build_client, exchange_token, gen_authorize_url, get_user_profile, GoogleRedirectExtractor,
};
use crate::conduit::users::find_or_create;
use crate::Repo;

#[derive(Serialize)]
struct AuthenticatedUser {
    user: User,
    token: String,
}

pub fn google_authorize_handler(state: State) -> (State, Response<Body>) {
    let google_client = build_client();
    let (authorize_url, _) = gen_authorize_url(google_client);

    let res = create_temporary_redirect(&state, authorize_url.to_string());

    (state, res)
}

pub fn google_redirect_handler(mut state: State) -> Box<HandlerFuture> {
    let query_param = GoogleRedirectExtractor::take_from(&mut state);
    let google_client = build_client();
    let token = exchange_token(&query_param, &google_client);
    let profile = get_user_profile(&token).expect("Couldn't get user's profile");
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
        Err(_e) => {
            let res = create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR);
            future::ok((state, res))
        }
    });

    Box::new(results)
}
