use rocket::http::ContentType;
use rocket::tokio::fs::File;
use lrtool::AdobeRocket;

#[rocket::get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

/*

    "/Users/gmadrid/Dropbox/Images/Adult/Images/2012/2012-07-01/tumblr_nxsue83hq01s1w2q9o1_1280.jpg"
    "/Users/gmadrid/Dropbox/Images/Adult/Images/2012/2012-07-24/tumblr_n5dawnNNxt1shb5ymo1_1280.jpg"
    "/Users/gmadrid/Dropbox/Images/Inbox/il_1140xN.2610095372_kiw2.jpg"

 */

#[rocket::get("/image")]
async fn image() -> (ContentType, File) {
    let file = File::open(
        "/Users/gmadrid/Dropbox/Images/Adult/Images/2012/2012-07-01/tumblr_nxsue83hq01s1w2q9o1_1280.jpg"
    ).await.expect("Cannot open image file.");
    (ContentType::JPEG, file)
}

#[rocket::main]
async fn main() {
    rocket::build()
        .build_adobe()
        .mount("/", rocket::routes!(image, index,))
        .ignite()
        .await
        .expect("PROBLEM WITH IGNITE")
        .launch()
        .await
        .expect("PROBLUM WITH LAUNCH");
}
