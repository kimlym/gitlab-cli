use super::file;
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct GitlabConfig {
    pub url: String,
    pub token: String,
}

impl GitlabConfig {
    pub fn new() -> GitlabConfig {
        GitlabConfig {
            url: String::from(""),
            token: String::from(""),
        }
    }
}

#[derive(Debug)]
pub struct Configator<'a> {
    save_file: &'a str,
}

impl<'a> Configator<'a> {
    pub fn new(save_file: &'a str) -> Configator {
        Configator { save_file }
    }

    pub fn read_config(&self) -> GitlabConfig {
        let config = match file::read_file(&self.save_file) {
            Ok(contents) => serde_json::from_str(&contents).unwrap(),
            Err(_) => self.create_config(),
        };

        config
    }

    pub fn write_config(&self, config: GitlabConfig) -> GitlabConfig {
        let config_content = serde_json::to_string(&config).unwrap();
        file::write_file(&self.save_file, &config_content).unwrap();
        config
    }

    pub fn create_config(&self) -> GitlabConfig {
        self.write_config(GitlabConfig::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_and_return_config_successfully() {
        let mut target = GitlabConfig::new();
        target.url = String::from("test url");
        target.token = String::from("test token");

        let configator = Configator::new("./resources/read_config.json");

        let config = configator.read_config();
        assert_eq!(target, config);
    }

    #[test]
    fn should_write_config_successfully() {
        let mut config = GitlabConfig::new();
        config.url = String::from("test url");
        config.url = String::from("test token");

        let configator = Configator::new("./resources/write_config.json");
        configator.write_config(config);
    }
}
