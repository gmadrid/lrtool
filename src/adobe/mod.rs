use crate::adobe::oauth2::AdobeOAuthState;
use reqwest::header::CONTENT_TYPE;
use reqwest::RequestBuilder;
use rocket::http::CookieJar;
use rocket::serde::Deserialize;
use rocket::{Build, Rocket, State};
use std::env;

mod oauth2;

pub trait AdobeRocket {
    fn build_adobe(self) -> Self;
}

impl AdobeRocket for Rocket<Build> {
    fn build_adobe(self) -> Self {
        oauth2::attach_oauth2(self)
            .mount("/adobe", rocket::routes!(adobe_health, adobe_entitlement))
    }
}

fn build_adobe_oauth_request(
    url: &str,
    client_id: &str,
    user_token: Option<&str>,
) -> RequestBuilder {
    let builder = reqwest::Client::new()
        .get(url)
        .header("x-api-key", client_id)
        .header(CONTENT_TYPE, "application/json");

    if let Some(token) = user_token {
        builder.bearer_auth(token)
    } else {
        builder
    }
}

#[rocket::get("/health")]
async fn adobe_health(state: &State<AdobeOAuthState>) -> () {
    let response =
        build_adobe_oauth_request("http://lr.adobe.io/v2/health", &state.client_id, None)
            .send()
            .await
            .unwrap()
            .text()
            .await;
    println!("{:?}", response);
}

#[derive(Debug, Deserialize)]
struct EntitlementResponse {
    base: String,
    id: String,
    #[serde(rename = "type")]
    typ: String,
    email: String,
    entitlement: EntitlementDetail,
}

#[derive(Debug, Deserialize)]
struct EntitlementDetail {
    status: String,
}

fn deserialize_adobe_response_body<'a, T>(body: &'a str) -> T
where
    T: Deserialize<'a>,
{
    if let Some(idx) = body.find('\n') {
        let json = &body[idx + 1..];
        let obj: T = serde_json::from_str(json).expect("coundn't parse body");
        return obj;
    } else {
        // TODO: get rid of this panic!
        panic!("What the?");
    }
}

#[rocket::get("/entitlement")]
async fn adobe_entitlement(state: &State<AdobeOAuthState>, cookies: &CookieJar<'_>) -> () {
    let token = cookies.get_private("token").expect("MISSING TOKEN");

    let response = build_adobe_oauth_request(
        "https://lr.adobe.io/v2/account",
        &state.client_id,
        Some(&token.value()),
    )
    .send()
    .await
    .unwrap()
    .text()
    .await;

    if let Ok(body) = response {
        let entitlement: EntitlementResponse = deserialize_adobe_response_body(&body);
        println!("ENTITLEMENT: {:?}", entitlement);
    }
}
