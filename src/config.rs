use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  pub certificate_path: String,
  pub key_path: String,
  pub hosts: IndexMap<String, String>,
  pub rules: IndexMap<String, String>,
}

pub fn parse() -> std::result::Result<Config, Box<dyn std::error::Error>> {
  // Open file
  let cwd = env::current_dir()?;
  let config_filename = ".proxee.json";
  let contents = fs::read_to_string(cwd.join(config_filename))?;
  Ok(serde_json::from_str(contents.as_str())?)
}
