use anyhow::Result;
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use oauth2::prelude::*;
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, RequestTokenError, Scope, StandardTokenResponse, TokenResponse, TokenUrl,
};
use std::env;
use std::str;
use url::Url;

use super::get_url;

pub fn build_facebook_client() -> BasicClient {
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

pub fn gen_facebook_authorize_url(client: BasicClient) -> (url::Url, CsrfToken) {
    client.authorize_url(CsrfToken::new_random)
}

pub fn exchange_facebook_token(
    extractor: &FacebookRedirectExtractor,
    client: &BasicClient,
) -> BasicToken {
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

pub fn get_facebook_user_profile(token: &BasicToken) -> Result<FacebookProfile> {
    // let token_header = format!("Bearer {}", token.access_token().secret());

    let headers = HeaderMap::new();
    // headers.insert(AUTHORIZATION, HeaderValue::from_str(&token_header).unwrap());

    let url = String::from(format!(
        "https://graph.facebook.com/me?fields=id,first_name,last_name,middle_name,gender,picture,email&access_token={}",
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

type BasicToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

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
