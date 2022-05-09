use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let host: String = "0.0.0.0".to_string();
        let port: String = "8080".to_string();

        Config {
            database_url,
            host,
            port,
        }
    }
}
