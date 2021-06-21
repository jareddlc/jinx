use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;

use super::log_exit;
use crate::file::{get_jinx_files, JinxFiles};
use crate::service::JinxService;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JinxConf {
  pub nginx_user: String,
  pub nginx_worker_processes: u8,
  pub nginx_worker_connections: u16,
  pub jinx_services: Vec<JinxService>,
}

impl Default for JinxConf {
  fn default() -> Self {
    Self {
      nginx_user: "nginx".to_string(),
      nginx_worker_processes: 1,
      nginx_worker_connections: 1024,
      jinx_services: vec![],
    }
  }
}

pub fn open_jinx_conf(jinx_files: &JinxFiles) -> File {
  // try to open jinx_conf
  let jinx_conf_file = match File::open(&jinx_files.jinx_conf) {
    Err(err) => {
      // create default jinx_conf if not found
      if err.kind() == ErrorKind::NotFound {
        // create jinx directory
        let _dir = match fs::create_dir_all(&jinx_files.jinx_home) {
          Err(err) => log_exit!("[JINX] Failed to create jinx directory", err),
          Ok(dir) => dir,
        };

        // create default jinx_conf
        let jinx_conf = JinxConf {
          ..Default::default()
        };
        let json = json!(jinx_conf);

        // write file
        fs::write(&jinx_files.jinx_conf, &json.to_string().as_bytes())
          .expect("[CONF] Failed to write jinx_conf");

        // return file
        match File::open(&jinx_files.jinx_conf) {
          Err(err) => log_exit!("[CONF] Failed to open jinx_conf", err),
          Ok(file) => file,
        }
      } else {
        log_exit!("[CONF] Failed to open jinx_conf", err)
      }
    }
    Ok(file) => file,
  };

  jinx_conf_file
}

// returns JinxConf
pub fn get_jinx_conf() -> JinxConf {
  // get jinx files
  let jinx_files = get_jinx_files();

  // open jinx conf
  let jinx_conf_file = open_jinx_conf(&jinx_files);

  // read the file
  let reader = BufReader::new(jinx_conf_file);

  // parse jinx_conf.json into a JinxConf
  let jinx_conf: JinxConf = match serde_json::from_reader(reader) {
    Err(err) => log_exit!("[CONF] Failed to parse jinx_conf", err),
    Ok(file) => file,
  };

  jinx_conf
}

// writes JinxConf to jinx_conf file
pub fn write_jinx_conf(jinx_conf: &JinxConf) {
  // get jinx files
  let jinx_files = get_jinx_files();

  // ensure path exists
  let _jinx_conf_file = open_jinx_conf(&jinx_files);

  // convert jinx_conf to JSON
  let json = json!(jinx_conf);

  // write the file
  fs::write(&jinx_files.jinx_conf, &json.to_string().as_bytes())
    .expect("[CONF] Failed to write jinx_conf");
}
