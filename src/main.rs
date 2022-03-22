use std::env;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use oauth2::basic::BasicClient;

fn main() {

    let client_id = ClientId::new(
        env::var("ADOBE_CLIENT_ID").expect("Missing the ADOBE_CLIENT_ID env var.")
    );
    let client_secret = ClientSecret::new(
        env::var("ADOBE_CLIENT_SECRET")
            .expect("Missing the ADOBE_CLIENT_SECRET env var.")
    );
    let auth_url = AuthUrl::new("https://ims-na1.adobelogin.com/ims/authorize/v2".to_string())
    .expect("invalid auth endpoint URL");

    let token_url = TokenUrl::new("https://ims-na1.adobelogin.com/ims/token/v3".to_string())
        .expect("invalid token endpoint URL");

    let client = BasicClient::new(
        client_id, Some(client_secret),
        auth_url, Some(token_url),
    )
        .set_redirect_uri(RedirectUrl::new("https://localhost:7777".to_string()).expect("Invalid redirect URL"));


    let (authorize_url, csrf_state) = client.authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("lr_partner_apis".to_string()))
        .add_scope(Scope::new("lr_partner_rendition_apis".to_string()))
        .url();

    println!("AUTH URL: {}", authorize_url);

    open::that(authorize_url.as_str());

//	open::that("http://www.instagram.com/").expect("failed");
//    println!("Hello, world!");
}
