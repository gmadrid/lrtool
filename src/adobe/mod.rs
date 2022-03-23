use crate::adobe::oauth2::AdobeOAuthState;
use reqwest::header::CONTENT_TYPE;
use rocket::http::CookieJar;
use rocket::{Build, Rocket, State};
use std::env;

mod oauth2;

pub trait AdobeServer {
    fn build_adobe(self) -> Self;
}

impl AdobeServer for Rocket<Build> {
    fn build_adobe(self) -> Self {
        oauth2::attach_oauth2(self)
            .mount("/adobe", rocket::routes!(adobe_health, adobe_entitlement))
    }
}

#[rocket::get("/health")]
async fn adobe_health(state: &State<AdobeOAuthState>) -> () {
    let client = reqwest::Client::new();
    let response = client
        .get("http://lr.adobe.io/v2/health")
        .header("x-api-key", &state.client_id)
        .send()
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}

#[rocket::get("/entitlement")]
async fn adobe_entitlement(cookies: &CookieJar<'_>) -> () {
    let token = cookies.get_private("token").expect("MISSING TOKEN");
    let client = reqwest::Client::new();
    let response = client
        .get("https://lr.adobe.io/v2/account")
        // Skipping the "token=" at the beginning.
        .bearer_auth(token.to_string()[6..].to_string())
        .header(
            "x-api-key",
            env::var("ADOBE_CLIENT_ID").expect("Missing the ADOBE_CLIENT_ID env var."),
        )
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}
