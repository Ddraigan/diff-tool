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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_new() -> Result<()> {
        // Create a mock configuration file
        let config_path = Path::new("config.toml");
        let mut file = File::create(&config_path)?;
        write!(file, "[keymap]\n\"key1\" = \"Quit\"")?;

        // Call the function
        let config = Config::new()?;

        // Check the result
        assert_eq!(config.keymap.0.get("key1"), Some(&Message::Quit));

        // Clean up
        std::fs::remove_file(config_path)?;

        Ok(())
    }
}
