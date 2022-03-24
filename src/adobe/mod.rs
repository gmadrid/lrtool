use crate::adobe::oauth2::AdobeOAuthState;
use reqwest::header::CONTENT_TYPE;
use reqwest::RequestBuilder;
use rocket::http::CookieJar;
use rocket::{Build, Rocket, State};
use crate::adobe::client::AdobeClient;

mod client;
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

#[rocket::get("/entitlement")]
async fn adobe_entitlement(adobe: AdobeClient) {
    let entitlement = adobe.entitlement().await;
    println!("SPECIAL ENTITLEMENT: {:?}", entitlement);
}
