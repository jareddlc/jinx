use handlebars::Handlebars;
use std::fs::File;
use std::str;

use crate::jinx;
use crate::jinx::Jinx;

pub fn render_template(jinx: Jinx) -> String {
    // create handlebars instance
    let handlebars = Handlebars::new();

    // load template from binary
    let nginx_bytes = include_bytes!("./templates/nginx.hbs");
    let nginx_template = str::from_utf8(nginx_bytes).expect("[NGINX] Failed to convert template");

    // render template
    let rendered_nginx = handlebars
        .render_template(nginx_template, &jinx)
        .expect("[NGINX] Failed to render template");

    rendered_nginx
}

pub fn create_nginx_conf(jinx: &Jinx) {
    // create handlebars instance
    let handlebars = Handlebars::new();

    // load template from binary
    let nginx_bytes = include_bytes!("./templates/nginx.hbs");
    let nginx_template = str::from_utf8(nginx_bytes).expect("[NGINX] Failed to convert template");

    // get jinx directories
    let jinx_directories = jinx::get_jinx_directories();
    let nginx_conf_dir = format!("{}/nginx.conf", jinx_directories.jinx_dir);

    // create output files
    let output_file = match File::create(nginx_conf_dir) {
        Err(_err) => panic!("[NGINX] Failed to create nginx.conf"),
        Ok(file) => file,
    };

    // output file
    handlebars
        .render_template_to_write(&nginx_template, &jinx, output_file)
        .expect("[MAIN] Failed to render template to file");
}
