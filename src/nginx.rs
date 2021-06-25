use handlebars::Handlebars;
use std::fs;
use std::fs::File;
use std::str;

use super::log_exit;
use crate::conf::JinxConf;
use crate::file::get_jinx_files;

// returns str of the nginx.hbs template
fn get_nginx_template() -> &'static str {
    // load template from binary
    let nginx_bytes = include_bytes!("./templates/nginx.hbs");
    let nginx_template = str::from_utf8(nginx_bytes).expect("[NGINX] Failed to convert template");

    nginx_template
}

// returns a rendered string of the nginx.hbs template
pub fn render_template(jinx_conf: &JinxConf) -> String {
    // create handlebars instance
    let handlebars = Handlebars::new();

    // load template
    let nginx_template = get_nginx_template();

    // render template
    let rendered_nginx = handlebars
        .render_template(nginx_template, &jinx_conf)
        .expect("[NGINX] Failed to render template");

    rendered_nginx
}

// writes JinxConf to nginx_conf file
pub fn write_nginx_conf(jinx_conf: &JinxConf) {
    // create handlebars instance
    let handlebars = Handlebars::new();

    // load template
    let nginx_template = get_nginx_template();

    // get jinx files
    let jinx_files = get_jinx_files();

    // create output files
    let output_file = match File::create(jinx_files.nginx_conf) {
        Err(err) => log_exit!("[NGINX] Failed to create nginx_conf", err),
        Ok(file) => file,
    };

    // write file
    handlebars
        .render_template_to_write(&nginx_template, &jinx_conf, output_file)
        .expect("[NGINX] Failed to write nginx_conf");
}

// writes the Dockerfile for jinx_loadbalancer
pub fn write_nginx_dockerfile() {
    // get jinx files
    let jinx_files = get_jinx_files();

    // load template from binary
    let dockerfile_bytes = include_bytes!("./templates/Dockerfile");

    // write file
    fs::write(
        &format!("{}/Dockerfile", jinx_files.jinx_home),
        &dockerfile_bytes,
    )
    .expect("[CONF] Failed to write jinx Dockerfile");
}
