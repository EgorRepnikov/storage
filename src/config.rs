use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub server: String,
    pub storage_dir: String,
    pub images_storage_dir: String,
    pub auth_credentials: String
}

lazy_static! {
    pub static ref CONFIG: Config = prepare_config();
}

fn prepare_config() -> Config {
    dotenv::dotenv().ok();
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error)
    }
}
