use crate::adobe::client::response::{
    AssetResponse, RetrieveAssetResponse, RetrieveAssetsResponse, RetrieveCatalogResponse,
};
use crate::adobe::oauth2::AdobeOAuthState;
use reqwest::header::CONTENT_TYPE;
use reqwest::RequestBuilder;
use response::EntitlementResponse;
use rocket::http::CookieJar;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::de::DeserializeOwned;
use serde::Deserialize;

mod response;

#[derive(Debug)]
pub struct AdobeClient {
    // TODO: avoid copying these Strings by making them &'a str.
    client_id: String,
    user_token: String,

    spew: bool,
}

fn deserialize_adobe_response_body<'a, T>(body: &'a str) -> T
where
    T: Deserialize<'a>,
{
    if let Some(idx) = body.find('\n') {
        let json: &'a str = &body[idx + 1..];
        let obj: T = serde_json::from_str(json).expect("coundn't parse body");
        return obj;
    } else {
        // TODO: get rid of this panic!
        panic!("What the?");
    }
}

impl AdobeClient {
    pub fn spew_next(&mut self) {
        self.spew = true;
    }

    fn build_get_request(&self, url: &str) -> RequestBuilder {
        self.add_basic_headers(reqwest::Client::new().get(url))
    }

    fn build_post_request(&self, url: &str) -> RequestBuilder {
        self.add_basic_headers(reqwest::Client::new().post(url))
    }

    fn add_basic_headers(&self, builder: RequestBuilder) -> RequestBuilder {
        builder
            .header("x-api-key", &self.client_id)
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&self.user_token)
    }

    async fn send_request<'d, T>(&mut self, uri: &'d str) -> T
    where
        T: DeserializeOwned,
    {
        let response = self
            .build_get_request(uri)
            .send()
            .await
            .unwrap()
            .text()
            .await;
        if let Ok(body) = response {
            if self.spew {
                eprintln!("SPEWING: {:?}", body);
                self.spew = true;
            }
            return deserialize_adobe_response_body::<T>(&body);
        }
        // TODO: fix the damn error handling
        panic!("WHAT!");
    }

    async fn send_binary_request(&mut self, uri: &str) -> Vec<u8> {
        let response = self
            .build_get_request(uri)
            .send()
            .await
            .unwrap()
            .text()
            .await;
        if let Ok(body) = response {
            if self.spew {
                eprintln!("SPEWING: {:?}", body);
                self.spew = true;
            }
            return body.into_bytes();
        }
        // TODO: fix the damn error handling
        panic!("WHAT!");
    }

    pub async fn entitlement(&mut self) -> EntitlementResponse {
        self.send_request("https://lr.adobe.io/v2/account").await
    }

    pub async fn retrieve_catalog(&mut self) -> RetrieveCatalogResponse {
        self.send_request("https://lr.adobe.io/v2/catalog").await
    }

    pub async fn retrieve_asset(&mut self, catalog_id: &str, asset_id: &str) -> AssetResponse {
        let template = "https://lr.adobe.io/v2/catalogs/{catalog_id}/assets/{asset_id}";
        let uri = template
            .replace("{catalog_id}", catalog_id)
            .replace("{asset_id}", asset_id);
        println!("RETASS url: {}", uri);
        self.send_request(&uri).await
    }

    // TODO: make spew be an internal mutation.
    pub async fn retrieve_assets(&mut self, catalog_id: &str) -> RetrieveAssetsResponse {
        let mut template = "https://lr.adobe.io/v2/catalogs/{catalog_id}/assets";
        let uri = template.replace("{catalog_id}", catalog_id);
        self.send_request(&uri).await
    }

    pub async fn generate_renditions(&mut self, catalog_id: &str, asset_id: &str) {
        // TODO: error checking!
        let mut template =
            "https://lr.adobe.io/v2/catalogs/{catalog_id}/assets/{asset_id}/renditions";
        let uri = template
            .replace("{catalog_id}", catalog_id)
            .replace("{asset_id}", asset_id);
        println!("GEN URI: {}", uri);

        let foo = dbg!(self
            .build_post_request(&uri)
            .header("x-generate-renditions", "fullsize,2560"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or("NOTHING".to_string());

        println!("GENERATE:\n{}", foo);
    }

    // TODO :make this return something that is not all the bytes.
    // TODO: make "fullsize" an enum.
    pub async fn retrieve_rendition(
        &mut self,
        catalog_id: &str,
        asset_id: &str,
        rendition_type: &str,
    ) -> Vec<u8> {
        let mut template = "https://lr.adobe.io/v2/catalogs/{catalog_id}/assets/{asset_id}/renditions/{rendition_type}";
        let uri = template
            .replace("{catalog_id}", catalog_id)
            .replace("{asset_id}", asset_id)
            .replace("{rendition_type}", rendition_type);

        println!("RENDITION URI: {}", uri);
        self.send_binary_request(&uri).await
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdobeClient {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // TODO: so much unmanaged error handling.
        let state = request
            .rocket()
            .state::<AdobeOAuthState>()
            .expect("FIX THIS SHIT");
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("Couldn't get cookies");

        let client_id = &state.client_id;
        // TODO: write this without the to_string()
        let user_token = cookies
            .get_private("token")
            .expect("MISSING TOKEN")
            .value()
            .to_string();

        Outcome::Success(AdobeClient {
            // TODO: get rid of this copy.
            client_id: client_id.to_string(),
            user_token: user_token,

            spew: false,
        })
    }
}
