#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;
use rs_docker::Docker;
use std::env;
use std::fs::File;
use std::str;

pub mod jinx;

use jinx::{Jinx, JinxService};

// logs errors and exits.
macro_rules! log_exit {
    ($($x:expr),+) => {
        {
            $(eprintln!("[Error] {}", $x);)+
            std::process::exit(1)
        }
    };
}

fn main() {
    // load logger
    if env::var("RUST_LOG").is_err() {
        // limit RUST_LOG to our project
        let log_level = format!("jinx={}", "debug");
        env::set_var("RUST_LOG", log_level);
    }

    // load jinx file
    debug!("[Main] Loading jinx file");
    let _jinx_file: Jinx = jinx::read_jinx_file();

    let jinx_service: JinxService = match jinx::get_jinx_service() {
        None => log_exit!("Failed to load jinx.json"),
        Some(svc) => svc,
    };

    // create handlebars instance
    let handlebars = Handlebars::new();

    // create template data
    let jinx = Jinx {
        services: vec![jinx_service],
        ..Default::default()
    };

    // load template from binary
    let nginx_bytes = include_bytes!("./templates/nginx.hbs");
    let nginx_template = str::from_utf8(nginx_bytes).unwrap();

    // render template
    let _rendered_nginx = handlebars
        .render_template(nginx_template, &jinx)
        .expect("[Main] Failed to render template");

    // create output files
    let output_file = match File::create("./nginx.conf") {
        Err(err) => log_exit!("Failed to create nginx.conf", &err),
        Ok(file) => file,
    };

    // output file
    handlebars
        .render_template_to_write(nginx_template, &jinx, output_file)
        .expect("[Main] Failed to render template to file");

    let mut _docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(err) => log_exit!("Failed to connect to docker socket", &err),
    };
}
