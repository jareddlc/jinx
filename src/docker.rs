use bollard::image::BuildImageOptions;
use bollard::network::CreateNetworkOptions;
use bollard::service::{
    EndpointPortConfig, EndpointSpec, NetworkAttachmentConfig, ServiceSpec, ServiceSpecMode,
    ServiceSpecModeReplicated, TaskSpec, TaskSpecContainerSpec,
};
use bollard::Docker;
use futures_util::stream::StreamExt;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::log_exit;
use crate::service::JinxService;

// builds the provided tar.gz file with meta from the JinxService
pub async fn build_docker_image(client: Docker, jinx_service: &JinxService, bytes: Vec<u8>) {
    // define image options
    let config = BuildImageOptions {
        dockerfile: "Dockerfile",
        t: &jinx_service.image_name,
        ..Default::default()
    };

    let mut image_build_stream = client.build_image(config, None, Some(bytes.into()));

    while let Some(msg) = image_build_stream.next().await {
        let message = match msg {
            Ok(msg) => msg,
            Err(err) => log_exit!("[DOCKER] Failed to get build message", err),
        };
        let stream = match message.stream {
            Some(stream) => stream,
            None => "".to_string(),
        };

        print!("{}", stream);
    }
}

// creates a docker network
pub async fn create_jinx_network(client: Docker) {
    // define jinx network
    let config = CreateNetworkOptions {
        name: "jinx_network",
        check_duplicate: true,
        driver: "overlay",
        internal: false,
        ..Default::default()
    };

    let network_id = match client.create_network(config).await {
        Ok(id) => id,
        Err(err) => log_exit!("[DOCKER] Failed to create jinx_network", err),
    };

    println!("Jinx network created: {:?}", network_id);
}

// returns a Docker client
pub fn get_client() -> Docker {
    let docker = match Docker::connect_with_socket_defaults() {
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

pub async fn create_service(client: Docker, jinx_service: &JinxService) {
    // create service name with jinx tag
    let name = format!("{}{}", &jinx_service.name, "-jinx".to_string());

    _create_service(client, jinx_service, name, None).await;
}

pub async fn create_jinx_loadbalancer_service(client: Docker, jinx_service: &JinxService) {
    // create jinx loadbalancer service
    let name = "jinx-loadbalancer".to_string();
    let publish_port = Some(jinx_service.image_port as i64);

    _create_service(client, jinx_service, name, publish_port).await;
}

async fn _create_service(
    client: Docker,
    jinx_service: &JinxService,
    name: String,
    published_port: Option<i64>,
) {
    // define network to attach service
    let networks = vec![NetworkAttachmentConfig {
        target: Some("jinx_network".to_string()),
        ..Default::default()
    }];

    // define service ports
    let endpoint_spec = EndpointSpec {
        ports: Some(vec![EndpointPortConfig {
            target_port: Some(jinx_service.image_port as i64),
            published_port: published_port,
            ..Default::default()
        }]),
        ..Default::default()
    };

    // define service
    let service = ServiceSpec {
        name: Some(name),
        mode: Some(ServiceSpecMode {
            replicated: Some(ServiceSpecModeReplicated { replicas: Some(1) }),
            ..Default::default()
        }),
        task_template: Some(TaskSpec {
            container_spec: Some(TaskSpecContainerSpec {
                image: Some(jinx_service.image_name.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        networks: Some(networks),
        endpoint_spec: Some(endpoint_spec),
        ..Default::default()
    };

    let service = match client.create_service(service, None).await {
        Ok(svc) => svc,
        Err(err) => log_exit!("[DOCKER] Failed to create jinx service", err),
    };
    let service_id = match service.id {
        Some(id) => id,
        None => "".to_string(),
    };

    println!("Jinx service created: {}", service_id);
}
