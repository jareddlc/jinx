use dirs;
use serde_derive::{Deserialize, Serialize};

use super::log_exit;

// Struct that contains Jinx home, Jinx configuration, and Nginx configuration paths
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JinxFiles {
  pub jinx_home: String,
  pub jinx_conf: String,
  pub nginx_conf: String,
}

// returns JinxFiles
pub fn get_jinx_files() -> JinxFiles {
  // get users home directory
  let home_dir = match dirs::home_dir() {
    None => log_exit!("[JINX] Failed to get home directory"),
    Some(dir) => dir,
  };

  // create jinx file paths
  let jinx_home = format!("{}/.jinx", home_dir.display());
  let jinx_conf = format!("{}/jinx_conf.json", jinx_home);
  let nginx_conf = format!("{}/nginx.conf", jinx_home);

  JinxFiles {
    jinx_home,
    jinx_conf,
    nginx_conf,
  }
}
