use dirs;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::Path;
use std::str;

use super::log_exit;

#[derive(Debug, Deserialize, Serialize, Clone, std::cmp::PartialEq)]
pub struct JinxService {
    pub name: Option<String>,
    pub project_dir: Option<String>,
    pub domain: Option<String>,
    pub container_name: Option<String>,
    pub container_port: Option<u16>,
    pub container_image: Option<String>,
    pub host_port: Option<u16>,
    pub entrypoint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Jinx {
    pub nginx_user: String,
    pub nginx_worker_processes: u8,
    pub nginx_worker_connections: u16,
    pub services: Vec<JinxService>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JinxDirectories {
    pub jinx_dir: String,
    pub jinx_file_dir: String,
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

pub fn create_jinx_file() -> File {
    // get jinx directories
    let jinx_directories = get_jinx_directories();

    // create jinx directory
    let _dir = match fs::create_dir_all(jinx_directories.jinx_dir) {
        Err(err) => log_exit!("[JINX] Failed to create jinx directory", err),
        Ok(dir) => dir,
    };

    // create a scope to close the file after writing
    {
        // create jinx file
        let jinx_file = File::create(&jinx_directories.jinx_file_dir)
            .expect("[JINX] Failed to create jinx file");

        // create default file
        let jinx = Jinx {
            ..Default::default()
        };

        // write file
        let empty_json = json!(jinx);
        serde_json::ser::to_writer(&jinx_file, &empty_json)
            .expect("[JINX] Failed to write jinx file");
    }

    open_jinx_file()
}

pub fn get_dockerfile() {
    // // get current directory
    // let current_dir = env::current_dir().expect("[JINX] Failed to get current directory");

    // // read the Dockerfile
    // let dockerfile_path = format!("{}/Dockerfile", current_dir.display());
    // let dockerfile = match fs::read_to_string(dockerfile_path) {
    //     Err(_err) => return None,
    //     Ok(file) => file,
    // };

    // // convert to vector of bytes
    // let dockerfile_bytes: Vec<u8> = dockerfile.as_bytes().iter().cloned().collect();

    // let mut service_dockerfile: JinxService = service.unwrap();
    // service_dockerfile.dockerfile = Some(dockerfile_bytes);
}

pub fn get_jinx_directories() -> JinxDirectories {
    // get users home directory
    let home_dir = match dirs::home_dir() {
        None => log_exit!("[JINX] Failed to get home directory"),
        Some(dir) => dir,
    };

    // create jinx paths
    let jinx_dir = format!("{}/.jinx", home_dir.display());
    let jinx_file_dir = format!("{}/jinx.json", jinx_dir);

    JinxDirectories {
        jinx_dir,
        jinx_file_dir,
    }
}

pub fn get_jinx_file() -> Jinx {
    // open jinx file
    let jinx_file = open_jinx_file();

    // read the file
    let reader = BufReader::new(jinx_file);

    // parse jinx.json into a JinxService
    let jinx: Jinx = match serde_json::from_reader(reader) {
        Err(err) => log_exit!("[JINX] Failed to parse jinx.json", err),
        Ok(file) => file,
    };

    jinx
}

pub fn get_jinx_service() -> Option<JinxService> {
    // get current directory
    let current_dir = env::current_dir().expect("[JINX] Failed to get current directory");

    // attempt to open jinx.json in current directory
    let jinx_path = format!("{}/jinx.json", current_dir.display());
    let file = match File::open(jinx_path) {
        Err(_err) => return None,
        Ok(file) => file,
    };

    // read the file
    let reader = BufReader::new(file);

    // parse jinx.json into a JinxService
    let service = match serde_json::from_reader(reader) {
        Err(_err) => None,
        Ok(file) => Some(file),
    };

    service
}

pub fn open_jinx_file() -> File {
    // get jinx directories
    let jinx_directories = get_jinx_directories();

    // open jinx file
    let jinx_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .open(Path::new(&jinx_directories.jinx_file_dir))
    {
        Err(_) => create_jinx_file(),
        Ok(file) => file,
    };

    jinx_file
}

pub fn save_jinx_file(jinx: &Jinx) {
    // open jinx file
    let jinx_file = open_jinx_file();

    // convert jinx to JSON
    let json = json!(jinx);

    // write the file
    serde_json::ser::to_writer(&jinx_file, &json).expect("[JINX] Failed to write jinx file");
}
