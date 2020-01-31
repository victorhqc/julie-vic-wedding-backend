use futures::{Future, Stream};
use gotham::helpers::http::response::create_empty_response;
use gotham::handler::{HandlerError, IntoHandlerError};
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};
use std::str::from_utf8;

pub fn extract_json<T>(state: &mut State) -> impl Future<Item = T, Error = HandlerError>
where
    T: serde::de::DeserializeOwned,
{
    Body::take_from(state)
        .concat2()
        .map_err(bad_request)
        .and_then(|body| {
            let b = body.to_vec();
            from_utf8(&b)
                .map_err(bad_request)
                .and_then(|s| serde_json::from_str::<T>(s).map_err(bad_request))
        })
}

pub fn bad_request<E>(e: E) -> HandlerError
where
    E: std::error::Error + Send + 'static,
{
    e.into_handler_error().with_status(StatusCode::BAD_REQUEST)
}

pub fn index_handler(state: State) -> (State, Response<Body>) {
    let res = create_empty_response(&state, StatusCode::OK);

    (state, res)
}
