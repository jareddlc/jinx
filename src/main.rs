use handlebars::Handlebars;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::str;

pub mod structs;

use structs::{Jinx, JinxService};

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
    // get current directory
    let current_dir = env::current_dir().expect("[Main] Failed to get current directory");

    // attempt to open jinx.json in current directory
    let jinx_path = format!("{}/jinx.json", current_dir.display());
    let file = match File::open(jinx_path) {
        Err(err) => log_exit!("Failed to load jinx.json", &err),
        Ok(file) => file,
    };

    // read the file
    let reader = BufReader::new(file);

    // parse jinx.json into a JinxService
    let service: JinxService = match serde_json::from_reader(reader) {
        Err(err) => log_exit!("Failed to parse jinx.json", &err),
        Ok(file) => file,
    };

    // create handlebars instance
    let handlebars = Handlebars::new();

    // create template data
    let jinx = Jinx {
        services: vec![service],
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
}
