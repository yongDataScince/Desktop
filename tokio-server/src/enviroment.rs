use std::{env};
use dotenv::dotenv;
use colored::Colorize;
use crate::errors::EnvError;

#[derive(Clone, Copy)]
pub enum Mode {
    Development,
    Production
}

#[derive(Clone)]
pub struct Enviroment {
    mode: Mode,
    pub host: String,
    pub port: String
}

impl Enviroment {
    fn init() -> Result<Self, EnvError> {
        dotenv().expect(".env file was not provided");

        return Ok(Enviroment {
            mode: Self::parse_mode(env::var("MODE").expect("MODE not provided")),
            host: env::var("HOST").expect("HOST not provided"),
            port: env::var("PORT").expect("PORT not provided")
        });
    }

    pub fn get_env() -> Result<Self, EnvError> {
        let res = match Self::init() {
            Ok(env) => env,
            Err(err) => panic!("{:?}", err),
        };

        match res.mode {
            Mode::Development => println!("{}", format!("using DEVELOPING mode").on_yellow()),
            Mode::Production => println!("{}", format!("using PRODUCTION mode").on_blue()),
        };

        Ok(res)
    }

    pub fn stringify_mode(mode: Mode) -> String {
        match mode {
            Mode::Development => String::from("development"),
            Mode::Production => String::from("production")
        }
    }

    pub fn parse_mode(mode_str: String) -> Mode {
        match mode_str {
            mode if mode == "development".to_string() => Mode::Development,
            mode if mode == "production".to_string() => Mode::Production,
            _ => panic!("unexpected mode") 
        }
    }
}

