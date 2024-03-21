use std::{collections::HashMap, ops::Deref};

use anyhow::Result;
use serde::Deserialize;

use crate::update::message::Message;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    keymap: KeyMap,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct KeyMap(pub HashMap<String, Message>);

impl Deref for KeyMap {
    type Target = HashMap<String, Message>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
