use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use crate::update::message::Message;

type KeyMap = HashMap<String, Message>;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    keymap: HashMap<String, Message>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::new("config", config::FileFormat::Toml))
            .build()?
            .try_deserialize()?;
        Ok(config)
    }

    pub fn keymap(&self) -> &KeyMap {
        &self.keymap
    }
}
