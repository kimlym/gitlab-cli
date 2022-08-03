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

fn get_config_fil_path() -> String {
    format!("{}/.gitlab-cli/config.json", var("HOME").unwrap())
}

pub fn read_config() -> GitlabConfig {
    let config = match file::read_file(&get_config_fil_path()) {
        Ok(contents) => serde_json::from_str(&contents).unwrap(),
        Err(_) => create_config(),
    };

    config
}

pub fn write_config(config: GitlabConfig) -> GitlabConfig {
    let config_content = serde_json::to_string(&config).unwrap();
    file::write_file(&get_config_fil_path(), &config_content).unwrap();
    config
}

fn create_config() -> GitlabConfig {
    write_config(GitlabConfig::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_and_return_config_successfully() {
        let mut target = GitlabConfig::new();
        target.url = String::from("test url");
        target.url = String::from("test token");

        let config = read_config();
        assert_eq!(target, config);
    }

    #[test]
    fn should_write_config_successfully() {
        let mut config = GitlabConfig::new();
        config.url = String::from("test url");
        config.url = String::from("test token");

        write_config(config);
    }
}
