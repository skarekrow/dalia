use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn version() -> impl Responder {
    // TODO: use config.toml to get API version information or something here
    HttpResponse::Ok().body("API Version Information.")
}

#[get("/weather")]
async fn weather() -> impl Responder {
    // TODO: Get ZIP Code from GET request and reach out to WeatherAPI
    //       Need to use WeatherAPI key from config.toml
    HttpResponse::Ok().body("Retrieving weather!")
}

async fn health() -> impl Responder {
    // TODO: Not in the scope of hackathon but leaving this here
    //       for the purpose of giving an example of adding additional routes
    HttpResponse::Ok().body("It Lives!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(version)
            .service(weather)
            .route("/health", web::get().to(health))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
