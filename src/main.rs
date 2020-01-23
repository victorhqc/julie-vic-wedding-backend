#[macro_use]
extern crate diesel;

use gotham::helpers::http::response::create_empty_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;
use hyper::{Body, Method, Response, StatusCode};
mod api;
mod models;
mod schema;
mod services;


fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

fn router() -> Router {
    build_simple_router(|route| {
        route
            .request(vec![Method::GET, Method::HEAD], "/")
            .to(index_handler);
    })
}

fn index_handler(state: State) -> (State, Response<Body>) {
    let res = create_empty_response(&state, StatusCode::OK);

    (state, res)
}
