use rs_docker::network::NetworkCreate;
use rs_docker::Docker;

use super::log_exit;
use crate::jinx::JinxService;

pub fn build_docker_image(mut client: Docker, jinx_service: JinxService) {
    // if jinx_service.dockerfile.is_none() {
    //     return;
    // }

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
