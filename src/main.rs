mod structs;

use envconfig::Envconfig;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "DALIA_APIKEY", default = "REPLACE_ME")]
    pub api_key: String,
}

fn main() {
    // Initialize config from environment variables
    let config = Config::init_from_env().unwrap();

    println!("Hello, world!");
    println!("{}", config.api_key);
}
