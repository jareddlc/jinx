use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JinxService {
  name: Option<String>,
  domain: Option<String>,
  container_name: Option<String>,
  container_port: Option<u16>,
  container_image: Option<String>,
  host_port: Option<u16>,
  entrypoint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Jinx {
  pub nginx_user: String,
  pub nginx_worker_processes: u8,
  pub nginx_worker_connections: u16,
  pub services: Vec<JinxService>,
}

impl Default for Jinx {
  fn default() -> Self {
    Self {
      nginx_user: "nginx".to_string(),
      nginx_worker_processes: 1,
      nginx_worker_connections: 1024,
      services: vec![],
    }
  }
}
