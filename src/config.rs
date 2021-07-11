use std::{fs, env, io};
use crate::{cli::DotArgs, error::{DotCliError, DotCliResult}};
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct DotConfig {
    email: String,
    debug: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DotCredentials {
    username: String,
    api_key: String,
}

impl DotConfig {

    pub fn init() -> DotCliResult<Self> {
        let conf = Self::from_rc().unwrap_or_default();
        let args = Self::from_args()?;
        Ok(conf)
    }

    pub fn from_rc() -> DotCliResult<Self> {
        let conf = fs::read_to_string("~/.config/dot/dot.toml")?;
        let conf: Self = toml::from_str(&conf.as_str())
            .expect("Could not serialize config file");
        Ok(Self::default())
    }

    pub fn from_args() -> DotCliResult<Self> {
        let args = DotArgs::from_args()?;
        let email = String::new();
        Ok(Self::default())
    }
}
