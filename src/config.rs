use std::path::PathBuf;

use anyhow::{anyhow, Result};
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

#[derive(EnumString, EnumVariantNames, Display)]
pub enum AppConfigVariableName {
    #[strum(serialize = "BSKY_IDENTIFIER")]
    BskyIdentifier,

    #[strum(serialize = "BSKY_PASSWORD")]
    BskyPassword,
}

pub struct AppConfig;

impl AppConfig {
    pub fn load() -> Result<PathBuf> {
        dotenv::dotenv()
            .map(|result| {
                for variable_name in AppConfigVariableName::VARIANTS {
                    if let Err(_) = std::env::var(variable_name) {
                        panic!("Env variable {:?} is not defined", variable_name)
                    }
                }
                result
            })
            .map_err(|e| anyhow!(e))
    }

    pub fn get(name: AppConfigVariableName) -> String {
        std::env::var(name.to_string()).expect(&format!(
            "Env variable {:?} is not defined, did you run AppConfig::check_integrity()?",
            name.to_string()
        ))
    }
}
