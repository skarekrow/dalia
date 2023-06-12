use std::env;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

mod structs;
use envconfig::Envconfig;
use structs::Root;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "DALIA_APIKEY", default = "REPLACE_ME")]
    pub api_key: String,
    #[envconfig(from = "TZ", default = "Chicago")]
    pub tz: String,
}

#[get("/")]
async fn version() -> impl Responder {
    // TODO: use config.toml to get API version information or something here
    HttpResponse::Ok().body("API Version Information.")
}

#[get("/current_weather")]
async fn current_weather() -> impl Responder {
    // TODO: Ugly, let's figure out config better
    let config = Config::init_from_env().unwrap();
    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={apikey}&q={timezone}&aqi=no",
        apikey = config.api_key,
        timezone = config.tz
    );
    let response = reqwest::get(&url).await;
    let weather: Root = response.unwrap().json().await.unwrap();

    HttpResponse::Ok().json(weather)
}

async fn health() -> impl Responder {
    // TODO: Not in the scope of hackathon but leaving this here
    //       for the purpose of giving an example of adding additional routes
    HttpResponse::Ok().body("It Lives!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(version)
            .service(current_weather)
            .route("/health", web::get().to(health))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
