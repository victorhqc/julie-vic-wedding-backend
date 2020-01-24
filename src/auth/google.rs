use oauth2::prelude::*;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
    TokenResponse,
    basic::{BasicClient, BasicTokenType}, StandardTokenResponse, EmptyExtraTokenFields
};
use std::env;
use url::Url;
use anyhow::Result;
use http::{HeaderMap, HeaderValue, header::{AUTHORIZATION}};

const GOOGLE_PEOPLE_ENDPOINT: &'static str = "https://www.googleapis.com";

pub fn build_google_client() -> BasicClient {
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID environment variable."),
    );

    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing GOOGLE_CLIENT_SECRET environment variable."),
    );

    let auth_url = AuthUrl::new(
        Url::parse("https://accounts.google.com/o/oauth2/v2/auth")
            .expect("Invalid authorization endpoint URL"),
    );

    let token_url = TokenUrl::new(
        Url::parse("https://www.googleapis.com/oauth2/v3/token")
            .expect("Invalid token endpoint URL"),
    );

    BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url)
    )
    .add_scope(Scope::new(
        "https://www.googleapis.com/auth/userinfo.email".to_string()
    ))
    .add_scope(Scope::new(
        "https://www.googleapis.com/auth/userinfo.profile".to_string()
    ))
    .set_redirect_url(RedirectUrl::new(
        Url::parse("http://localhost:7878/google/redirect").expect("Invalid redirect URL"),
    ))
}

pub fn gen_authorize_url(client: BasicClient) -> (url::Url, CsrfToken) {
    client.authorize_url(CsrfToken::new_random)
}

pub fn exchange_token(extractor: &GoogleRedirectExtractor, client: &BasicClient) -> BasicToken {
    let code = AuthorizationCode::new(extractor.code.to_owned());

    let token = client.exchange_code(code).expect("Couldn't exchange token");

    token
}

pub fn get_user_profile(token: &BasicToken) -> Result<GoogleProfile> {
    let token_header = format!("Bearer {}", token.access_token().secret());
    println!("{}", token_header);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token_header).unwrap());

    let url = format!("{}/oauth2/v1/userinfo?alt=json", GOOGLE_PEOPLE_ENDPOINT);

    let client = reqwest::Client::new();
    let mut response = client
        .get(&url)
        .headers(headers)
        .send()?;

    let profile: GoogleProfile = response.json()?;

    Ok(profile)
}

#[derive(Deserialize, Serialize, StateData, StaticResponseExtender)]
pub struct GoogleRedirectExtractor {
    state: String,
    code: String,
    scope: Vec<String>,
    prompt: String,
    session_state: String
}

pub type BasicToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleProfile {
    id: String,
    email: String,
    family_name: Option<String>,
    gender: Option<String>,
    given_name: Option<String>,
    locale: Option<String>,
    picture: Option<String>,
    verified_email: bool,
}
