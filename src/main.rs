use lrtool::AdobeServer;

#[rocket::get("/hello/<name>")]
async fn hello(name: Option<&str>) -> String {
    format!("Hello, {}", name.unwrap_or("DEFAULT"))
}

#[rocket::get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .build_adobe()
        .mount("/", rocket::routes!(index, hello,))
        .ignite()
        .await
        .expect("PROBLEM WITH IGNITE")
        .launch()
        .await
        .expect("PROBLUM WITH LAUNCH");
}
