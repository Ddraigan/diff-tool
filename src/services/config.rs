use crate::update::message::Message;
use anyhow::{bail, Result};
use directories::ProjectDirs;
use serde::Deserialize;
use std::path::PathBuf;
use std::{collections::HashMap, ops::Deref};

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    keymap: KeyMap,
    // TODO: Colour schemes
    // colour_scheme: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct KeyMap(HashMap<String, Message>);

impl Deref for KeyMap {
    type Target = HashMap<String, Message>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_dir = get_config_dir()?; // Assuming you have a function to get the config directory

        // Specify the path to the config file
        let config_path = config_dir.join("config.toml");

        let config = config::Config::builder()
            .add_source(config::File::from(config_path.clone()))
            .build()
            .unwrap_or(Self::default_config()?);

        let config = config.try_deserialize()?;

        Ok(config)
    }

    pub fn keymap(&self) -> &KeyMap {
        &self.keymap
    }

    fn default_config() -> Result<config::Config, config::ConfigError> {
        let default_keymap = r#"[keymap]
"esc" = "Quit"
"q" = "Quit"
"ctrl+c" = "Quit"
"ctrl+d" = "Quit"
"Shift+g" = "LastRow"
"g" = "FirstRow"
"j" = "NextRow"
"k" = "PrevRow"

[colour_scheme]
"fg" = "white"
"bg" = "transparent"
"text" = "white""#;

        config::Config::builder()
            .add_source(config::File::from_str(
                default_keymap,
                config::FileFormat::Toml,
            ))
            .build()
    }
}

// INFO: probably won't need this
// fn get_data_dir() -> Result<PathBuf> {
//     let directory = if let Ok(s) = std::env::var("DIFF-TOOL-DATA") {
//         PathBuf::from(s)
//     } else if let Some(proj_dirs) = ProjectDirs::from("", "ddraigan", "diff-tool") {
//         proj_dirs.data_local_dir().to_path_buf()
//     } else {
//         bail!("Unable to find data directory for Diff-Tool");
//     };
//     Ok(directory)
// }

fn get_config_dir() -> Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("DIFF-TOOL-CONFIG") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("", "ddraigan", "diff-tool") {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        bail!("Unable to find config directory for Diff-Tool")
    };
    Ok(directory)
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
        write!(
            file,
            "[keymap]\n\"key1\" = \"Quit\"\n\n[colour_scheme]\n\"key1\" = \"Quit\""
        )?;

        // Call the function
        let config = Config::new()?;

        // Check the result
        assert_eq!(config.keymap.0.get("key1"), Some(&Message::Quit));

        // Clean up
        std::fs::remove_file(config_path)?;

        Ok(())
    }
}
