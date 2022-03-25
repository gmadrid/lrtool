use crate::adobe::client::AdobeClient;
use crate::adobe::oauth2::AdobeOAuthState;
use reqwest::header::CONTENT_TYPE;
use reqwest::RequestBuilder;
use rocket::http::{ContentType, CookieJar};
use rocket::{Build, Rocket, State};
use rocket::tokio::fs::File;

mod client;
mod oauth2;

pub trait AdobeRocket {
    fn build_adobe(self) -> Self;
}

impl AdobeRocket for Rocket<Build> {
    fn build_adobe(self) -> Self {
        oauth2::attach_oauth2(self).mount(
            "/adobe",
            rocket::routes!(adobe_catalog, adobe_health, adobe_entitlement, adobe_image),
        )
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
async fn adobe_entitlement(mut adobe: AdobeClient) -> String {
    let entitlement = adobe.entitlement().await;
    format!("{:?}", entitlement)
}

// TODO: no, seriously, make spew be internally mutable.
#[rocket::get("/catalog")]
async fn adobe_catalog(mut adobe: AdobeClient) -> String {
    let catalog = adobe.retrieve_catalog().await;
    format!("{:?}", catalog)
}

#[rocket::get("/image")]
async fn adobe_image(mut adobe: AdobeClient) -> Vec<u8> {
                                         //(ContentType, File) {
    let catalog = adobe.retrieve_catalog().await;
    println!("CATALOG: {:?}", catalog);
    let assets = adobe.retrieve_assets(&catalog.id).await;
    let asset = &assets.resources[0];
    println!("ASSET: {:?}", asset);

    let uri = assets.base + &asset.links.self_.href;
    println!("URI: {}", uri);

    adobe.generate_renditions(&catalog.id, &asset.id).await;

    let image_bytes = adobe.retrieve_rendition(&catalog.id, &asset.id , "fullsize").await;
    image_bytes
}