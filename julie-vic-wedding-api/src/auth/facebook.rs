use anyhow::Result;
use http::HeaderMap;
use julie_vic_wedding_core::models::NewUser;
use oauth2::prelude::*;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    RequestTokenError, Scope, TokenResponse, TokenUrl,
};
use std::env;
use std::str;
use url::Url;

use super::{BasicToken, Profile};
use crate::utils::get_url;

pub fn build_client() -> BasicClient {
    let facebook_client_id = ClientId::new(
        env::var("FACEBOOK_CLIENT_ID").expect("Missing FACEBOOK_CLIENT_ID environment variable."),
    );

    let facebook_client_secret = ClientSecret::new(
        env::var("FACEBOOK_CLIENT_SECRET")
            .expect("Missing FACEBOOK_CLIENT_SECRET environment variable."),
    );

    let auth_url = AuthUrl::new(
        Url::parse("https://www.facebook.com/v5.0/dialog/oauth")
            .expect("Invalid authorization endpoint URL"),
    );

    let token_url = TokenUrl::new(
        Url::parse("https://graph.facebook.com/v5.0/oauth/access_token")
            .expect("Invalid token endpoint URL"),
    );

    BasicClient::new(
        facebook_client_id,
        Some(facebook_client_secret),
        auth_url,
        Some(token_url),
    )
    .add_scope(Scope::new("email".to_string()))
    .set_redirect_url(RedirectUrl::new(
        Url::parse(format!("{}/facebook/redirect", get_url()).as_ref())
            .expect("Invalid redirect URL"),
    ))
}

pub fn gen_authorize_url(client: BasicClient) -> (url::Url, CsrfToken) {
    client.authorize_url(CsrfToken::new_random)
}

pub fn exchange_token(extractor: &FacebookRedirectExtractor, client: &BasicClient) -> BasicToken {
    let code = AuthorizationCode::new(extractor.code.to_owned());
    let token = client.exchange_code(code);

    match token {
        Ok(token) => token,
        Err(e) => match e {
            RequestTokenError::Parse(e, v) => {
                println!("E: {:?}", e);
                println!("V: {}", str::from_utf8(&v).unwrap());
                panic!("Can't parse exchange token!");
            }
            _ => {
                println!("{:?}", e);
                panic!("Can't exchange token!");
            }
        },
    }
}

pub fn get_user_profile(token: &BasicToken) -> Result<FacebookProfile> {
    let headers = HeaderMap::new();
    let fields = "id,first_name,last_name,middle_name,gender,picture,email";

    let url = String::from(format!(
        "https://graph.facebook.com/me?fields={}&access_token={}",
        fields,
        token.access_token().secret()
    ));

    println!("{}", url);
    let client = reqwest::Client::new();
    let mut response = client.get(&url).headers(headers).send()?;

    let profile: FacebookProfile = response.json()?;

    Ok(profile)
}

#[derive(Deserialize, Serialize, StateData, StaticResponseExtender)]
pub struct FacebookRedirectExtractor {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FacebookProfile {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub picture: Option<ProfilePicture>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfilePicture {
    pub data: PictureData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PictureData {
    pub height: i32,
    pub width: i32,
    pub is_silhouette: bool,
    pub url: String,
}

impl Profile for FacebookProfile {
    fn new_user(&self) -> NewUser {
        let first_name = self.first_name.clone();
        let last_name = self.last_name.clone();
        let email = self.email.clone();

        NewUser::new(first_name, last_name, email)
    }
}
