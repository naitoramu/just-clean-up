use std::env;

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
    pub token_secret: String,
}

impl AppConfig {

    pub fn get() -> &'static Self {
        if CONFIG.get().is_none() {
            CONFIG.set(AppConfig::new()).expect("Cannot create AppConfig");
        }

        return CONFIG.get().unwrap()
    }

    fn new() -> Self {
        Self {
            port: EnvVariable::required("PORT").as_u16(),
            base_path: EnvVariable::required("BASE_PATH").as_string(),
            development_mode: EnvVariable::or_default("DEV_MODE", "false").as_bool(),
            db_url: EnvVariable::required("DATABASE_URL").as_string(),
            token_secret: EnvVariable::required("JWT_SECRET").as_string(),
        }
    }
}

struct EnvVariable {
    var_name: String,
    string_value: String
}

impl EnvVariable {
    fn required(var_name: &str) -> Self {
        EnvVariable {
            var_name: var_name.to_string(),
            string_value: env::var(var_name)
                .expect(format!("Cannot read environment variable '{var_name}'").as_str())
        }
    }

    fn or_default(var_name: &str, default: &str) -> Self {
        EnvVariable {
            var_name: var_name.to_string(),
            string_value: env::var(var_name)
                .unwrap_or(default.to_string())
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