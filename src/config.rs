use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Config {
  pub username: String,
}


pub fn load_config() -> Config {
  if let Some(config_path) = get_config_path() {
    let content = fs::read_to_string(&config_path)
      .unwrap_or_else(|err| {
        eprintln!("Failed to read config file: {}", err);
        std::process::exit(1);
      });
    
    let config = toml::from_str(&content)
      .unwrap_or_else(|err| {
        eprintln!("Failed to parse TOML file: {}", err);
        std::process::exit(1);
      });

    config
  } else {
    init_config()
  }
}


fn get_config_path() -> Option<PathBuf> {
  let proj_dir = ProjectDirs::from("", "", "ucl-cs-ssh-tool")?;
  Some(proj_dir.config_dir().join("config.toml"))
}


fn init_config() -> Config {
  
}
