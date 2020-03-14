use rs_docker::Docker;

pub fn connect() -> Docker {
    let docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(_err) => panic!("[DOCKER] Failed to connect to docker socket"),
    };

    docker
}
