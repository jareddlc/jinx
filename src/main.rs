#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate tar;

use std::env;

pub mod commands;
pub mod docker;
pub mod jinx;
pub mod nginx;
pub mod targz;

// logs errors and exits
#[macro_export]
macro_rules! log_exit {
    ($($x:expr),+) => {
        {
            $(eprintln!("[ERROR] {}", $x);)+
            std::process::exit(1)
        }
    };
}

fn main() {
    // load logger
    if env::var("JINX_LOG").is_ok() {
        // limit RUST_LOG to our project
        let log_level = format!(
            "jinx={}",
            env::var("JINX_LOG").expect("[MAIN] Failed to get JINX_LOG")
        );
        env::set_var("RUST_LOG", log_level);
    }

    // initialize the logger
    env_logger::init();

    // parse arguments
    let args: Vec<String> = std::env::args().collect();

    // handle arguments
    commands::handle_args(args);
}
