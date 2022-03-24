use crate::adobe::oauth2::AdobeOAuthState;
use reqwest::header::CONTENT_TYPE;
use reqwest::RequestBuilder;
use rocket::http::CookieJar;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Debug)]
pub struct AdobeClient {
    // TODO: avoid copying these Strings by making them &'a str.
    client_id: String,
    user_token: String,
}

#[derive(Debug, Deserialize)]
pub struct EntitlementResponse {
    base: String,
    id: String,
    #[serde(rename = "type")]
    typ: String,
    email: String,
    entitlement: EntitlementDetail,
}

#[derive(Debug, Deserialize)]
pub struct EntitlementDetail {
    status: String,
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
    fn build_request(&self, url: &str) -> RequestBuilder {
        reqwest::Client::new()
            .get(url)
            .header("x-api-key", &self.client_id)
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&self.user_token)
    }

    async fn send_request<'d, T>(&self, uri: &'d str) -> T
    where
        T: DeserializeOwned,
    {
        let response = self.build_request(uri).send().await.unwrap().text().await;
        if let Ok(body) = response {
            return deserialize_adobe_response_body::<T>(&body);
        }
        // TODO: fix the damn error handling
        panic!("WHAT!");
    }

    pub async fn entitlement(&self) -> EntitlementResponse {
        self.send_request("https://lr.adobe.io/v2/account").await
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
        })
    }
}
