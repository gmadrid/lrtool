use lrtool::AdobeRocket;

#[rocket::get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .build_adobe()
        .mount("/", rocket::routes!(index,))
        .ignite()
        .await
        .expect("PROBLEM WITH IGNITE")
        .launch()
        .await
        .expect("PROBLUM WITH LAUNCH");
}
