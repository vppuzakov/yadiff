use clap::{load_yaml, App};
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub token: String,
    pub remote: String,
    pub local: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        let local = matches.value_of("local").unwrap();
        let remote = matches.value_of("remote").unwrap();
        let token = env::var("YADIFF_TOKEN").expect("Yandex disk token <YADIFF_TOKEN> not found");

        Config {
            token,
            local: local.to_string(),
            remote: remote.to_string(),
        }
    }
}
