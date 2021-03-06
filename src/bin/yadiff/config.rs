use clap::{load_yaml, App};

pub struct Config {
    pub token: String,
    pub remote: String,
    pub local: String,
    pub window: u32,
}

impl Config {
    pub fn new() -> Self {
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        let local = matches.value_of("local").unwrap();
        let remote = matches.value_of("remote").unwrap();
        let token = matches.value_of("token").unwrap();
        let window = matches.value_of("window").unwrap();

        Config {
            token: token.to_string(),
            local: local.to_string(),
            remote: remote.to_string(),
            window: window.parse().unwrap(),
        }
    }
}
