use futures::Future;
use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::{FromState, State};
use hyper::header::{
    HeaderMap, HeaderValue, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
    ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_MAX_AGE, ORIGIN,
};
use hyper::Method;
use std::option::Option;

#[derive(Clone, NewMiddleware, Debug, PartialEq)]
pub struct CorsMiddleware {
    methods: Vec<Method>,
    origin: Option<String>,
    max_age: u32,
}

impl CorsMiddleware {
    pub fn new(methods: Vec<Method>, origin: Option<String>, max_age: u32) -> Self {
        Self {
            methods,
            origin,
            max_age,
        }
    }

    pub fn default() -> Self {
        let methods = vec![
            Method::DELETE,
            Method::GET,
            Method::HEAD,
            Method::OPTIONS,
            Method::PATCH,
            Method::POST,
            Method::PUT,
        ];

        let origin = None;
        let max_age = 86400;

        Self::new(methods, origin, max_age)
    }
}

impl Middleware for CorsMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture> + Send + 'static,
    {
        let settings = self.clone();
        let f = chain(state).map(|(state, mut response)| {
            let origin: String;
            if settings.origin.is_none() {
                let origin_raw = HeaderMap::borrow_from(&state).get(ORIGIN).clone();
                let ori = match origin_raw {
                    Some(o) => o.to_str().unwrap().to_string(),
                    None => "*".to_string(),
                };

                origin = ori;
            } else {
                origin = settings.origin.unwrap();
            };

            let methods = settings
                .methods
                .iter()
                .map(|m| String::from(m.as_str()))
                .collect::<Vec<String>>()
                .join(", ");

            response.headers_mut().insert(
                ACCESS_CONTROL_ALLOW_CREDENTIALS,
                HeaderValue::from_str("true").unwrap(),
            );

            response.headers_mut().insert(
                ACCESS_CONTROL_ALLOW_HEADERS,
                HeaderValue::from_str("Authorization, Content-Type").unwrap(),
            );

            response.headers_mut().insert(
                ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_str(&origin).unwrap(),
            );

            response.headers_mut().insert(
                ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_str(&methods).unwrap(),
            );

            response
                .headers_mut()
                .insert(ACCESS_CONTROL_MAX_AGE, HeaderValue::from(settings.max_age));

            (state, response)
        });

        Box::new(f)
    }
}
