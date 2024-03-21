use std::env;

use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub admin_email: String,
    pub admin_password: String,
}

impl Config {
    pub fn init() -> Config {
        let environment_result = dotenv();

        if environment_result.is_err() {
            println!("🔥 Failed to load .env file.");
            std::process::exit(1);
        }

        let database_url = match env::var("DATABASE_URL") {
            Ok(jwt_secret) => Some(jwt_secret),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring DATABASE_URL environment variable: {}",
                    error
                );

                None
            }
        }
        .unwrap();

        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(jwt_secret) => Some(jwt_secret),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring JWT_SECRET environment variable: {}",
                    error
                );

                None
            }
        }
        .unwrap();

        let admin_email = match env::var("ADMIN_EMAIL") {
            Ok(admin_email) => Some(admin_email),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring ADMIN_EMAIL environment variable: {}",
                    error
                );

                None
            }
        }
        .unwrap();

        let admin_password = match env::var("ADMIN_PASSWORD") {
            Ok(admin_password) => Some(admin_password),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring ADMIN_PASSWORD environment variable: {}",
                    error
                );

                None
            }
        }
        .unwrap();

        Config {
            database_url,
            jwt_secret,
            admin_email,
            admin_password,
        }
    }
}
