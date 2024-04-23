use crate::update::message::Message;
use anyhow::{bail, Result};
use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    keymap: KeyMap,
    // TODO: Colour schemes
    // colour_scheme: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct KeyMap(HashMap<String, Message>);

impl std::ops::Deref for KeyMap {
    type Target = HashMap<String, Message>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let config_dir = get_config_dir()?;

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

    fn default_config() -> Result<config::Config> {
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

        let config = config::Config::builder()
            .add_source(config::File::from_str(
                default_keymap,
                config::FileFormat::Toml,
            ))
            .build()?;

        Ok(config)
    }
}

pub fn get_data_dir() -> Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("DIFF_TOOL_DATA") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("", "ddraigan", "diff-tool") {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        bail!("Unable to find data directory for Diff-Tool");
    };
    Ok(directory)
}

pub fn get_config_dir() -> Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("DIFF_TOOL_CONFIG") {
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
    fn test_config_dir() {
        let config_dir = get_config_dir().unwrap();
        let control =
            PathBuf::from("C:\\Users\\Web.RNW\\AppData\\Local\\ddraigan\\diff-tool\\config");

        assert_eq!(config_dir, control)
    }

    #[test]
    fn test_new() -> Result<()> {
        // Create a mock configuration file
        let config_path = Path::new("config.toml");
        let mut file = File::create(&config_path)?;
        write!(
            file,
            "[keymap]\n\"key1\" = \"Quit\"\n\n[colour_scheme]\n\"fg\" = \"white\""
        )?;

        // Call the function
        let config = AppConfig::new()?;

        // Check the result
        assert_eq!(config.keymap.0.get("key1"), Some(&Message::Quit));

        // Clean up
        std::fs::remove_file(config_path)?;

        Ok(())
    }
}
