use std::env;

use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use qstring::QString;

mod structs;
use envconfig::Envconfig;
use structs::CurrentWeather;

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
    let mut weather: CurrentWeather = response.unwrap().json().await.unwrap();

    if weather.location.tz_id.contains("America/") {
        weather.current.temp = Some(weather.current.temp_f);
        weather.current.wind_speed = Some(weather.current.wind_mph);
        weather.current.feels_like = Some(weather.current.feelslike_f);
        weather.current.gust = Some(weather.current.gust_mph);
    } else {
        weather.current.temp = Some(weather.current.temp_c);
        weather.current.wind_speed = Some(weather.current.wind_kph);
        weather.current.feels_like = Some(weather.current.feelslike_c);
        weather.current.gust = Some(weather.current.gust_kph);
    }

    HttpResponse::Ok().json(weather)
}

#[get("/weather")]
async fn weather_fn(req : HttpRequest) -> impl Responder {
    let query_str = req.query_string(); 
    let qs = QString::from(query_str); // deconstruct the query string for parameters
    let q_param_get = qs.get("q"); // REQUIRED q query param (zip .. TZ .. lat,long)
    let d_param_get = qs.get("d"); // d days forecast param, accepts number of days

    // If no q param was given, send a bad request response
    if q_param_get.is_none() {
        return HttpResponse::BadRequest().body("Invalid q param specificed.")
    }

    // Get q & d params
    let q_param = q_param_get.unwrap();
    let mut _apiendpoint = "current";
    let mut _forecast_query = String::new();

    // If the days forecast is specified
    // then change the endpoint and include the parameter
    if !d_param_get.is_none(){
        let d_param = d_param_get.unwrap();
        _apiendpoint = "forecast";
        _forecast_query = format!("&days={}", d_param);
    }

    // Get weatherapi response
    let config = Config::init_from_env().unwrap();
    let url = format!(
        "https://api.weatherapi.com/v1/{endpoint}.json?key={apikey}&q={api_query}{forecast_query}",
        endpoint = _apiendpoint,
        apikey = config.api_key,
        api_query = q_param,
        forecast_query = _forecast_query
    );

    // Use struct
    // TODO: This needs to be modified to include the new forecast
    let response = reqwest::get(&url).await;
    let weather: Root = response.unwrap().json().await.unwrap();

    // Return weather results
    return HttpResponse::Ok().json(weather)

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
            .service(weather_fn)
            .route("/health", web::get().to(health))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
