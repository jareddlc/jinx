use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;

use super::log_exit;

#[derive(Debug, Deserialize, Serialize, Clone, std::cmp::PartialEq)]
pub struct JinxService {
  pub name: String,
  pub domain: String,
  pub image_name: String,
  pub image_port: u64,
  pub image_env: Option<Vec<String>>,
  pub image_secrets: Option<Vec<String>>,
  pub https_redirect: bool,
  pub https: bool,
}

impl Default for JinxService {
  fn default() -> Self {
    Self {
      name: "None".to_string(),
      domain: "None".to_string(),
      image_name: "None".to_string(),
      image_port: 8080,
      image_env: None,
      image_secrets: None,
      https_redirect: false,
      https: false,
    }
  }
}

// returns Option<JinxService>
pub fn get_jinx_service() -> JinxService {
  // get current directory
  let current_dir = env::current_dir().expect("[JINX] Failed to get current directory");

  // attempt to open jinx.json in current directory
  let jinx_path = format!("{}/jinx.json", current_dir.display());
  let file = match File::open(jinx_path) {
    Err(err) => log_exit!("[SERVICE] Failed to open jinx.json {}", err),
    Ok(file) => file,
  };

  // read the file
  let reader = BufReader::new(file);

  // parse jinx.json into a JinxService
  let service = match serde_json::from_reader(reader) {
    Err(err) => log_exit!("[SERVICE] Failed to parse jinx.json {}", err),
    Ok(file) => file,
  };

  service
}

pub fn get_jinx_proxy_service() -> JinxService {
  JinxService {
    name: "jinx_proxy".to_string(),
    image_name: "jinx_proxy".to_string(),
    image_port: 80,
    ..Default::default()
  }
}
