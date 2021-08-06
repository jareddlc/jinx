use bollard::Docker;
use std::fs;

use super::log_exit;
use crate::docker::run_image;
use crate::file::get_jinx_files;
use crate::file::JinxFiles;

pub async fn run_letsencrypt_container(client: Docker, jinx_files: &JinxFiles, domain: String) {
    // certbot server ports
    let ports = vec!["80:80/tcp", "443:443/tcp"];

    // mount volumes
    let conf = format!("{}:/etc/letsencrypt", jinx_files.letsencrypt_conf);
    let www = format!("{}:/var/www/certbot", jinx_files.letsencrypt_www);
    let volumes = vec![conf.as_str(), www.as_str()];

    let std_domain = format!("-d {}", domain);
    let www_domain = format!("-d www.{}", domain);

    let cmds = vec![
        "certonly",
        "--register-unsafely-without-email",
        "--agree-tos",
        "--standalone",
        std_domain.as_str(),
        www_domain.as_str(),
    ];

    run_image(client, "certbot/certbot", ports, volumes, None, Some(cmds)).await
}

// writes the paths for letsencrypt to mount with nginx
pub fn write_letsencrypt() {
    // get jinx files
    let jinx_files = get_jinx_files();

    // create conf dir
    match fs::create_dir_all(jinx_files.letsencrypt_conf) {
        Ok(dir) => dir,
        Err(err) => log_exit!("[CERT] Failed to create letsencrypt conf directory", err),
    };

    // create www dir
    match fs::create_dir_all(jinx_files.letsencrypt_www) {
        Ok(dir) => dir,
        Err(err) => log_exit!("[CERT] Failed to create letsencrypt www directory", err),
    };
}
