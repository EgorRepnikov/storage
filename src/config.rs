#[derive(Clone, Deserialize, Debug)]
struct Config {
    pub server: String
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
