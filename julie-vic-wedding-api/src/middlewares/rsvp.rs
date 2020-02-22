use chrono::{NaiveDate, Utc};
use futures::future;
use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::create_empty_response;
use gotham::middleware::Middleware;
use gotham::state::State;
use hyper::StatusCode;
use std::env;
use std::str::FromStr;

#[derive(NewMiddleware, Copy, Clone)]
pub struct RsvpDateMiddleware;

impl Middleware for RsvpDateMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture> + Send + 'static,
    {
        let now = Utc::now().naive_local().date();
        let limit_date_str = env::var("RSVP_LIMIT_DATE").expect("Failed to parse RSVP date");
        let limit_date = NaiveDate::from_str(&limit_date_str).expect("RSVP is not a valid date");

        if now < limit_date {
            chain(state)
        } else {
            let response = create_empty_response(&state, StatusCode::FORBIDDEN);
            Box::new(future::ok((state, response)))
        }
    }
}
