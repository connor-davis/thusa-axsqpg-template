use std::env;

use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        let environment_result = dotenv();

        if environment_result.is_err() {
            println!("🔥 Failed to load .env file.");
            std::process::exit(1);
        }

        let database_url = env::var("DATABASE_URL");

        if database_url.is_err() {
            println!("🔥 Failed to load DATABASE_URL from .env file.");
            std::process::exit(1);
        }

        Config {
            database_url: database_url.unwrap(),
        }
    }
}
