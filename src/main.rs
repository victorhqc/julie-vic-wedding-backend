#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate gotham_derive;

use dotenv::dotenv;
use gotham::handler::{HandlerError, HandlerFuture};
use gotham::helpers::http::response::{
    create_empty_response, create_response, create_temporary_redirect,
};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};
use std::env;
use std::pin::Pin;

use futures::prelude::*;
use futures::{future, stream, Future, Stream};

mod api;
mod auth;
mod models;
mod schema;
mod services;

use auth::{
    build_google_client, exchange_token, gen_authorize_url, get_user_profile,
    GoogleRedirectExtractor,
};

fn main() {
    dotenv().ok();

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get_or_head("/").to(index_handler);

        route.get("/authorize").to(authorize_handler);
        route
            .get("/redirect")
            .with_query_string_extractor::<GoogleRedirectExtractor>()
            .to(redirect_handler);
    })
}

fn index_handler(state: State) -> (State, Response<Body>) {
    let res = create_empty_response(&state, StatusCode::OK);

    (state, res)
}

fn authorize_handler(state: State) -> (State, Response<Body>) {
    // TODO: Move to state.
    let google_client = build_google_client();
    let (authorize_url, _) = gen_authorize_url(google_client);

    let res = create_temporary_redirect(&state, authorize_url.to_string());

    (state, res)
}

fn redirect_handler(mut state: State) -> (State, Response<Body>) {
    let query_param = GoogleRedirectExtractor::take_from(&mut state);
    let google_client = build_google_client();
    let token = exchange_token(&query_param, &google_client);
    let profile = get_user_profile(&token).expect("Couldn't get user's profile");

    let res = create_response(
        &state,
        StatusCode::OK,
        mime::APPLICATION_JSON,
        serde_json::to_vec(&profile).expect("Couldn't serialize query param"),
    );

    (state, res)
}
