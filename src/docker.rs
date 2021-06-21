use rs_docker::container::ContainerCreate;
use rs_docker::network::NetworkCreate;
use rs_docker::Docker;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::log_exit;
use crate::service::JinxService;

// builds the provided tar.gz file with meta from the JinxService
pub fn build_docker_image(mut client: Docker, jinx_service: &JinxService, bytes: Vec<u8>) {
    // create image name with tag
    let name = format!("{}:{}", &jinx_service.name, "jinx".to_string());

    let _build_result = client
        .build_image(bytes, name)
        .expect("[DOCKER] Failed to build image");
}

// creates a docker network
pub fn create_jinx_network(mut client: Docker) {
    // define jinx network
    let network = NetworkCreate {
        Name: "jinx_network".to_string(),
        CheckDuplicate: Some(true),
        Driver: Some("overlay".to_string()),
        Internal: Some(false),
        Attachable: Some(false),
        Ingress: None,
        EnableIPv6: Some(false),
        Options: None,
        Labels: None,
    };

    // create docker network
    let network_id = match client.create_network(network) {
        Ok(id) => id,
        Err(err) => log_exit!("[DOCKER] Failed to create jinx_network", err),
    };

    println!("Jinx network created: {}", network_id);
}

// returns a Docker client
pub fn get_client() -> Docker {
    let docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(err) => log_exit!("[DOCKER] Failed to connect to docker socket", err),
    };

    docker
}

// returns a vector of lines from the .dockerignore file
pub fn get_dockerignore() -> Vec<String> {
    let mut lines = vec![];

    // get current directory
    let current_dir = env::current_dir().expect("[DOCKER] Failed to get current directory");

    // attempt to open .dockerignore in current directory
    let jinx_path = format!("{}/.dockerignore", current_dir.display());
    let file = match File::open(jinx_path) {
        Err(_err) => return lines,
        Ok(file) => file,
    };

    // read the file
    let reader = BufReader::new(file);

    // add lines to array
    for line in reader.lines() {
        let ln = match line {
            Err(err) => format!("Error: {}", err),
            Ok(line) => line,
        };
        lines.push(ln);
    }

    lines
}

pub fn run_image(mut client: Docker, jinx_service: &JinxService) {
    // pub ExposedPorts: Option<HashMap<String, HashMap<i32, i32>>>,
    // pub HostConfig: Option<HostConfigCreate>

    let container = ContainerCreate {
        ExposedPorts: None,
        HostConfig: None,
        Image: format!("{}:{}", &jinx_service.name, "jinx".to_string()),
        Labels: None,
    };
    client
        .create_container(jinx_service.name.clone(), container)
        .expect("[DOCKER] Failed to create container");
}
