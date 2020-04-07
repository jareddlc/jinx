use rs_docker::network::NetworkCreate;
use rs_docker::Docker;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::log_exit;
use crate::jinx::JinxService;

pub fn build_docker_image(_client: Docker, jinx_service: JinxService) {
    if jinx_service.name.is_none() {
        return;
    }

    // let dockerfile = match jinx_service.dockerfile {
    //     None => return,
    //     Some(bytes) => bytes,
    // };

    // let build_result = client.build_image(dockerfile, "jinx_tag".to_string());

    // debug!("[DOCKER] Build: {:?}", build_result);
}

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
        Err(err) => log_exit!("[DOCKER] Failed to create jinx network", err),
    };

    debug!("[DOCKER] Jinx network created: {}", network_id);
}

pub fn get_client() -> Docker {
    let docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(err) => log_exit!("[DOCKER] Failed to connect to docker socket", err),
    };

    docker
}

pub fn get_dockerignore() -> Vec<String> {
    let mut lines = vec![];

    // get current directory
    let current_dir = env::current_dir().expect("[DOCKER] Failed to get current directory");

    // attempt to open jinx.json in current directory
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
