use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use julie_vic_wedding_core::models::NewUser;
use oauth2::prelude::*;
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, Scope, StandardTokenResponse, TokenResponse, TokenUrl,
};
use std::env;
use std::result::Result;
use url::Url;

use super::error::AuthError;
use super::Profile;
use crate::utils::get_url;

const GOOGLE_PEOPLE_ENDPOINT: &str = "https://www.googleapis.com";

pub fn build_client() -> Result<BasicClient, AuthError> {
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

    let client = BasicClient::new(
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
    .set_redirect_url(RedirectUrl::new(Url::parse(
        format!("{}/google/redirect", get_url()).as_ref(),
    )?));

    Ok(client)
}

pub fn gen_authorize_url(client: BasicClient) -> (url::Url, CsrfToken) {
    client.authorize_url(CsrfToken::new_random)
}

pub fn exchange_token(
    extractor: &GoogleRedirectExtractor,
    client: &BasicClient,
) -> Result<BasicToken, AuthError> {
    let code = AuthorizationCode::new(extractor.code.to_owned());
    let token = client.exchange_code(code)?;

    Ok(token)
}

pub fn get_user_profile(token: &BasicToken) -> Result<GoogleProfile, AuthError> {
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
    authuser: i32,
}

type BasicToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

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

impl Profile for GoogleProfile {
    fn new_user(&self) -> NewUser {
        let name = self
            .given_name
            .as_ref()
            .unwrap_or(&String::from(""))
            .to_string();
        let family_name = self.family_name.clone();
        let email = self.email.clone();

        NewUser::new(name, family_name, email)
    }
}
