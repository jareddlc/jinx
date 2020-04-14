use crossterm::{
    execute,
    style::{Print, ResetColor},
};
use std::io::{stdout, Write};
use std::str;

use super::log_exit;
use crate::docker;
use crate::jinx;
use crate::jinx::{Jinx, JinxService};
use crate::nginx;
use crate::targz;

pub fn handle_args(args: Vec<String>) {
    // help
    if args.contains(&"-h".to_string()) {
        return help();
    }

    // version
    if args.contains(&"-v".to_string()) {
        return version();
    }

    // init
    if args.contains(&"init".to_string()) {
        return init();
    }

    // load
    if args.contains(&"load".to_string()) {
        return load();
    }

    // build
    if args.contains(&"build".to_string()) {
        return build();
    }

    // default
    help()
}

pub fn build() {}

pub fn load() {
    // load jinx.json from current directory
    debug!("[COMMANDS] Loading jinx.json from current directory");
    let jinx_service: JinxService = match jinx::get_jinx_service() {
        None => log_exit!("[COMMANDS] Failed to load jinx.json"),
        Some(svc) => svc,
    };

    // load jinx.json from jinx home directory
    debug!("[COMMANDS] Loading jinx.json");
    let mut jinx_file: Jinx = jinx::get_jinx_file();

    // append new jinx service
    if !jinx_file.services.contains(&jinx_service) {
        jinx_file.services.push(jinx_service.clone());
    }

    // save jinx
    debug!("[COMMANDS] Saving jinx.json");
    jinx::save_jinx_file(&jinx_file);

    // get dockerignore file
    debug!("[COMMANDS] Loading dockerignore");
    let dockerignore = docker::get_dockerignore();

    // create tar file of the project directory
    debug!("[COMMANDS] Creating tar of project directory");
    targz::create_tar(&jinx_service, &dockerignore);

    // get tar file bytes
    debug!("[COMMANDS] Opening tar file");
    let tar_bytes = targz::get_tar(&jinx_service);

    // get docker client
    debug!("[COMMANDS] Getting docker client");
    let client = docker::get_client();

    debug!("[COMMANDS] Build docker image");
    docker::build_docker_image(client, &jinx_service, tar_bytes);

    // create template data
    let _nginx_rendered = nginx::create_nginx_conf(&jinx_file);
}

pub fn help() {
    // load template from binary
    let help_bytes = include_bytes!("./templates/help.hbs");
    let help_template = str::from_utf8(help_bytes).expect("[COMMANDS] Failed to convert template");

    execute!(stdout(), Print(help_template), ResetColor).expect("[COMMANDS] Failed to run help");
}

pub fn init() {
    // get docker client
    let client = docker::get_client();

    // create jinx network
    docker::create_jinx_network(client);
}

pub fn version() {
    execute!(stdout(), Print("0.1.0\n"), ResetColor).expect("[COMMANDS] Failed to run help");
}
