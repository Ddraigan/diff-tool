use std::collections::HashMap;

use anyhow::Result;
use config::{Config, File, FileFormat};
use serde::Deserialize;

use crate::update::message::Message;

type KeyMap = HashMap<String, Message>;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    keymap: HashMap<String, Message>,
}

fn get_config() -> Result<AppConfig> {
    let config = Config::builder()
        .add_source(File::new("config", FileFormat::Toml))
        .build()?;

    let final_settings = config.try_deserialize()?;

    Ok(final_settings)
}

impl AppConfig {
    pub fn new() -> Self {
        let keymap = get_config().unwrap().keymap;

        Self { keymap }
    }

    pub fn keymap(&self) -> &KeyMap {
        &self.keymap
    }
}
