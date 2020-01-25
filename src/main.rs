#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate gotham_derive;

use dotenv::dotenv;
use gotham::helpers::http::response::{
    create_empty_response,
};
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{State};
use hyper::{Body, Response, StatusCode};

mod api;
mod auth;
mod models;
mod schema;
mod handlers;
mod users;

use api::Api;

use auth::{
    GoogleRedirectExtractor,
};

use handlers::auth::{google_redirect_handler, google_authorize_handler};

fn main() {
    dotenv().ok();

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

fn router() -> Router {
    // create the counter to share across handlers
    let api = Api::connect();

    // create our state middleware to share the counter
    let middleware = StateMiddleware::new(api);

    // create a middleware pipeline from our middleware
    let pipeline = single_middleware(middleware);

    // construct a basic chain from our pipeline
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route.get_or_head("/").to(index_handler);

        route.get("/google/authorize").to(google_authorize_handler);
        route
            .get("/google/redirect")
            .with_query_string_extractor::<GoogleRedirectExtractor>()
            .to(google_redirect_handler);
    })
}

fn index_handler(state: State) -> (State, Response<Body>) {
    let res = create_empty_response(&state, StatusCode::OK);

    (state, res)
}
