use dirs;
use std::fs;
use std::fs::File;
use std::path::Path;

pub struct Config {
    pub path: String,
    pub token: String,
}

impl Config {
    pub fn init() -> Config {
        let config_path = match dirs::home_dir() {
            Some(path) => format!("{}/{}", path.display(), ".sndcld-token"),
            None => panic!("Home directory not found!"),
        };

        let mut _config_token = String::from("");

        if !Path::new(&config_path).exists() {
            File::create(&config_path).unwrap();
        }

        _config_token = fs::read_to_string(&config_path).expect("");

        Config {
            path: config_path,
            token: _config_token,
        }
    }

    pub fn save_token(&self, token: String) {
        fs::write(&self.path, token).unwrap();
        println!("Token saved successfully!");
    }
}
