use rocket::fairing::AdHoc;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::{Build, Rocket};
use rocket_oauth2::{HyperRustlsAdapter, OAuth2, OAuthConfig, StaticProvider, TokenResponse};
use std::borrow::Cow;
use std::env;

struct Adobe;

// TODO: remove these 'pub'
pub struct AdobeOAuthState {
    pub client_id: String,
}

impl AdobeOAuthState {
    fn from_env() -> Self {
        AdobeOAuthState {
            client_id: client_id_from_env(),
        }
    }
}

static ADOBE_PROVIDER: StaticProvider = StaticProvider {
    auth_uri: Cow::Borrowed("https://ims-na1.adobelogin.com/ims/authorize/v2"),
    token_uri: Cow::Borrowed("https://ims-na1.adobelogin.com/ims/token/v3"),
};

pub fn attach_oauth2(rocket: Rocket<Build>) -> Rocket<Build> {
    let client_id = client_id_from_env();
    let client_secret = client_secret_from_env();
    rocket
        .manage(AdobeOAuthState::from_env())
        .attach(AdHoc::on_ignite("OAuth Config", |rocket| async {
            let provider = ADOBE_PROVIDER.clone();
            let redirect_uri = Some("https://localhost:7777/adobe/oauth2/callback".to_string());
            let config = OAuthConfig::new(provider, client_id, client_secret, redirect_uri);
            rocket.attach(OAuth2::<Adobe>::custom(
                HyperRustlsAdapter::default(),
                config,
            ))
        }))
        .mount(
            "/adobe/oauth2",
            rocket::routes!(adobe_login, adobe_callback),
        )
}

// TODO: do both of these using the Rocket config business.
fn client_id_from_env() -> String {
    env::var("ADOBE_CLIENT_ID").expect("Missing the ADOBE_CLIENT_ID env var.")
}

fn client_secret_from_env() -> String {
    env::var("ADOBE_CLIENT_SECRET").expect("Missing the ADOBE_CLIENT_SECRET env var.")
}

#[rocket::get("/login")]
async fn adobe_login(oauth2: OAuth2<Adobe>, mut cookies: &CookieJar<'_>) -> Redirect {
    oauth2
        .get_redirect(
            &mut cookies,
            &["openid", "lr_partner_apis", "lr_partner_rendition_apis"],
        )
        .unwrap()
}

#[rocket::get("/callback")]
async fn adobe_callback(token: TokenResponse<Adobe>, cookies: &CookieJar<'_>) -> Redirect {
    cookies.add_private(
        Cookie::build("token", token.access_token().to_string())
            .same_site(SameSite::Lax)
            .finish(),
    );
    Redirect::to("/")
}
