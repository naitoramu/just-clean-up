use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;
use tokio::sync::OnceCell;

lazy_static! {
    pub static ref CONFIG: OnceCell<AppConfig> = OnceCell::new();
}

#[derive(Debug)]
pub struct AppConfig {
    pub port: u16,
    pub base_path: String,
    pub development_mode: bool,
    pub db_url: String,
    pub db_username: String,
    pub db_password: String,
}

impl AppConfig {

    pub fn get() -> &'static Self {
        if CONFIG.get().is_none() {
            CONFIG.set(AppConfig::new()).expect("Cannot create AppConfig");
        }

        return CONFIG.get().unwrap()
    }

    fn new() -> Self {
        dotenv().ok();
        Self {
            port: EnvVariable::new("PORT").as_u16(),
            base_path: EnvVariable::new("BASE_PATH").as_string(),
            development_mode: EnvVariable::new("DEV_MODE").as_bool(),
            db_url: EnvVariable::new("DATABASE_URL").as_string(),
            db_username: EnvVariable::new("DATABASE_USERNAME").as_string(),
            db_password: EnvVariable::new("DATABASE_PASSWORD").as_string(),
        }
    }
}

struct EnvVariable {
    var_name: String,
    string_value: String
}

impl EnvVariable {
    fn new(var_name: &str) -> Self {
        EnvVariable {
            var_name: var_name.to_string(),
            string_value: env::var(var_name)
                .expect(format!("Cannot read environment variable '{var_name}'").as_str())
        }
    }

    fn as_string(&self) -> String {
        self.string_value.clone()
    }

    fn as_bool(&self) -> bool {
        self.string_value.parse()
            .expect(format!("{} is not a valid bool value", self.var_name)
                .as_str())
    }

    fn as_u16(&self) -> u16 {
        self.string_value.parse()
            .expect(format!("{} is not a valid u16 value", self.var_name)
                .as_str())
    }
}