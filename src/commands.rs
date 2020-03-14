use crossterm::{
    execute,
    style::{Print, ResetColor},
};
use std::io::{stdout, Write};

use crate::jinx;
use crate::jinx::{Jinx, JinxService};
use crate::nginx;

// logs errors and exits.
macro_rules! log_exit {
    ($($x:expr),+) => {
        {
            $(eprintln!("[ERROR] {}", $x);)+
            std::process::exit(1)
        }
    };
}

pub fn handle_args(args: Vec<String>) {
    if args.contains(&"-h".to_string()) {
        return help();
    }

    get_jinx_service();
}

pub fn get_jinx_service() {
    // load jinx.json from current directory
    debug!("[COMMANDS] Loading jinx.json from current directory");
    let jinx_service: JinxService = match jinx::get_jinx_service() {
        None => log_exit!("Failed to load jinx.json"),
        Some(svc) => svc,
    };

    // load jinx.json from jinx home directory
    debug!("[COMMANDS] Loading jinx.json");
    let mut jinx_file: Jinx = jinx::get_jinx_file();

    // append new jinx service
    if !jinx_file.services.contains(&jinx_service) {
        jinx_file.services.push(jinx_service);
    }

    // save jinx
    debug!("[COMMANDS] Saving jinx.json");
    jinx::save_jinx_file(&jinx_file);

    // create template data
    let _nginx_rendered = nginx::create_nginx_conf(&jinx_file);
}

pub fn help() {
    execute!(stdout(), Print("Jinx help"), ResetColor).expect("[COMMANDS] Failed to run help");
}
