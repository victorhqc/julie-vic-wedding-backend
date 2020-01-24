#[macro_use]
extern crate diesel;


use dotenv::dotenv;
use gotham::helpers::http::response::{create_empty_response, create_temporary_redirect};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use std::env;

mod api;
mod auth;
mod models;
mod schema;
mod services;

use auth::{build_google_client, gen_authorize_url};

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

    println!("{}", env::var("GOOGLE_CLIENT_ID").expect("oops"));

    let res = create_temporary_redirect(&state, authorize_url.to_string());

    (state, res)
}
