use crate::error::Codes;
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir_all};
use std::io::Write;
use std::path::Path;

//TODO: make a internal struct for aws configs to avoid same prefix
#[allow(clippy::struct_field_names)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub aws_region: String,
    pub aws_profile: String,
    pub aws_sg_id: String,
    pub aws_sg_rule_id: String,
}

impl Config {
    fn default() -> Self {
        Self {
            aws_region: "us-east-1".to_string(),
            aws_profile: "default".to_string(),
            aws_sg_id: "sg-123456".to_string(),
            aws_sg_rule_id: "sgr-123456".to_string(),
        }
    }
}

static USERLAND_CONFIG_PATH: &str = ".config/aws-sg-ip-sync/config.toml";

pub fn load(base_path_str: &str) -> Result<Option<Config>, Codes> {
    let base_path = Path::new(base_path_str);
    let config_path = base_path.join(USERLAND_CONFIG_PATH);

    if config_path.exists() {
        let config_string = fs::read_to_string(&config_path).map_err(|_| Codes::ConfigReadError)?;
        let parsed_result = toml::from_str(&config_string).map_err(|_| Codes::ConfigParseError)?;

        Ok(parsed_result)
    } else {
        let config = Config::default();
        if let Some(parent) = config_path.parent() {
            create_dir_all(parent).expect("Failed to create directories");
        }

        let toml = toml::to_string(&config).expect("Failed to serialize config");
        let mut file = fs::File::create(&config_path).expect("Failed to create config file");
        file.write_all(toml.as_bytes())
            .expect("Failed to write to config file");

        println!("A default config has been created at {}. Please configure it and run the program again.", config_path.display());
        Ok(None)
    }
}
