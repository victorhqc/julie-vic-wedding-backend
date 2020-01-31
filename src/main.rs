#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate gotham_derive;

use dotenv::dotenv;
use gotham::middleware::logger::RequestLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::set::{finalize_pipeline_set, new_pipeline_set};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham_middleware_diesel::{self, DieselMiddleware};
use gotham_middleware_jwt::JWTMiddleware;

mod attend_status_type;
mod auth;
mod conduit;
mod db;
mod handlers;
mod middlewares;
mod models;
mod schema;
mod utils;

use auth::facebook::FacebookRedirectExtractor;
use auth::google::GoogleRedirectExtractor;
use auth::{get_secret, AuthUser};
use db::{repo, Repo};
use handlers::auth::{
    facebook_authorize_handler, facebook_redirect_handler, google_authorize_handler,
    google_redirect_handler,
};
use handlers::index_handler;
use middlewares::rsvp::RsvpDateMiddleware;

fn main() {
    dotenv().ok();
    env_logger::init();

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

fn router() -> Router {
    let repo = repo();

    let pipelines = new_pipeline_set();
    let (pipelines, default) = pipelines.add(
        new_pipeline()
            .add(DieselMiddleware::new(repo))
            .add(RequestLogger::new(log::Level::Info))
            .build(),
    );
    let (pipelines, authenticated) = pipelines.add(
        new_pipeline()
            .add(JWTMiddleware::<AuthUser>::new(get_secret()).scheme("Bearer"))
            .build(),
    );
    let (pipelines, rsvp_check) = pipelines.add(new_pipeline().add(RsvpDateMiddleware).build());

    let pipeline_set = finalize_pipeline_set(pipelines);
    let default_chain = (default, ());
    let auth_chain = (authenticated, default_chain);
    let rsvp_chain = (rsvp_check, auth_chain);

    build_router(default_chain, pipeline_set, |route| {
        route.get_or_head("/").to(index_handler);

        route.get("/google/authorize").to(google_authorize_handler);
        route
            .get("/google/redirect")
            .with_query_string_extractor::<GoogleRedirectExtractor>()
            .to(google_redirect_handler);

        route
            .get("/facebook/authorize")
            .to(facebook_authorize_handler);
        route
            .get("/facebook/redirect")
            .with_query_string_extractor::<FacebookRedirectExtractor>()
            .to(facebook_redirect_handler);

        route.scope("/api", |route| {
            route.with_pipeline_chain(auth_chain, |route| {
                route.get("/me").to(handlers::users::me);
            });

            route.with_pipeline_chain(rsvp_chain, |route| {
                route.post("/rsvp").to(handlers::users::rsvp);
            });
        })
    })
}
