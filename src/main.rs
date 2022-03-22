use rocket::{get, launch, routes};

#[get("/hello/<name>")]
async fn hello(name: Option<&str>) -> String {
    format!("Hello, {}", name.unwrap_or("DEFAULT"))
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}
