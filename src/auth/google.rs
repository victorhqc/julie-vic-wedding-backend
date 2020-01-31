use anyhow::Result;
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use oauth2::prelude::*;
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, Scope, StandardTokenResponse, TokenResponse, TokenUrl,
};
use std::env;
use url::Url;
use super::get_url;

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
        Some(token_url),
    )
    .add_scope(Scope::new(
        "https://www.googleapis.com/auth/userinfo.email".to_string(),
    ))
    .add_scope(Scope::new(
        "https://www.googleapis.com/auth/userinfo.profile".to_string(),
    ))
    .set_redirect_url(RedirectUrl::new(
        Url::parse(format!("{}/google/redirect", get_url()).as_ref())
            .expect("Invalid redirect URL"),
    ))
}

pub fn gen_google_authorize_url(client: BasicClient) -> (url::Url, CsrfToken) {
    client.authorize_url(CsrfToken::new_random)
}

pub fn exchange_google_token(
    extractor: &GoogleRedirectExtractor,
    client: &BasicClient,
) -> BasicToken {
    let code = AuthorizationCode::new(extractor.code.to_owned());

    let token = client.exchange_code(code).expect("Couldn't exchange token");

    token
}

pub fn get_google_user_profile(token: &BasicToken) -> Result<GoogleProfile> {
    let token_header = format!("Bearer {}", token.access_token().secret());

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token_header).unwrap());

    let url = format!("{}/oauth2/v1/userinfo?alt=json", GOOGLE_PEOPLE_ENDPOINT);

    let client = reqwest::Client::new();
    let mut response = client.get(&url).headers(headers).send()?;

    let profile: GoogleProfile = response.json()?;

    Ok(profile)
}

#[derive(Deserialize, Serialize, StateData, StaticResponseExtender)]
pub struct GoogleRedirectExtractor {
    state: String,
    code: String,
    scope: Vec<String>,
    prompt: String,
    session_state: String,
}

type BasicToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleProfile {
    pub id: String,
    pub email: String,
    pub family_name: Option<String>,
    pub gender: Option<String>,
    pub given_name: Option<String>,
    pub locale: Option<String>,
    pub picture: Option<String>,
    pub verified_email: bool,
}
