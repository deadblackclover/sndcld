use dirs;
use std::fs;
use std::fs::File;
use std::path::Path;

/// Application configuration
pub struct Config {
    /// Application settings file path
    pub path: String,
    /// Token for Soundcloud API
    pub token: String,
}

impl Config {
    /// Init config
    pub fn init() -> Config {
        let config_path = match dirs::home_dir() {
            Some(path) => format!("{}/{}", path.display(), ".sndcld-token"),
            None => panic!("Home directory not found!"),
        };

        let mut _config_token = String::from("");

        if !Path::new(&config_path).exists() {
            File::create(&config_path).expect("Failed to create file");
        }

        _config_token = fs::read_to_string(&config_path).expect("Failed to read string");

        Config {
            path: config_path,
            token: _config_token,
        }
    }

    /// Save token for Soundcloud API
    pub fn save_token(&self, token: String) -> Result<(), std::io::Error> {
        fs::write(&self.path, token)?;
        println!("Token saved successfully!");
        Ok(())
    }
}
