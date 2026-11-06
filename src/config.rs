use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;


#[derive(Serialize, Deserialize)]
pub struct Config {
  pub username: String,
}


impl Config {
  fn save(&self) -> io::Result<()> {
    let config_path = get_config_path()
      .ok_or_else(|| io::Error::other("Could not determine config directory"))?;

    // Create directories if they don't exist
    fs::create_dir_all(config_path.parent().unwrap())?;

    let toml_string = toml::to_string(self)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    fs::write(config_path, toml_string)?;
    Ok(())
  }
}


pub fn load_config() -> Config {
  let Some(config_path) = get_config_path().filter(|p| p.is_file()) else {
    return init_config().unwrap_or_else(|e| {
      eprintln!("Critical IO error during setup: {}", e);
      std::process::exit(1);
    });
  };

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
}


fn get_config_path() -> Option<PathBuf> {
  let proj_dir = ProjectDirs::from("", "", "ucl-cs-ssh-tool")?;
  Some(proj_dir.config_dir().join("config.toml"))
}


fn init_config() -> io::Result<Config> {
  println!("Config not found. Let's get you set up!");

  let mut username = String::new();
  print!("Enter your CS lab username: ");
  let _ = io::stdout().flush()?;
  
  io::stdin().read_line(&mut username)?;
  let username = username.trim().to_string();

  let config = Config { username };
  if let Err(err) = config.save() {
    eprintln!("Failed to save config: {}", err);
    eprintln!("Proceeding without saving config")
  };
  
  Ok(config)
}
