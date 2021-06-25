use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use tar::Builder;

use super::log_exit;
use crate::file::get_jinx_files;
use crate::service::get_jinx_loadbalancer_service;
use crate::service::JinxService;

pub fn create_jinx_loadbalancer_tar() {
  // get service
  let jinx_service = get_jinx_loadbalancer_service();

  // get jinx files
  let jinx_files = get_jinx_files();

  let excluded = vec!["jinx_conf.json".to_string(), ".jinx.tar.gz".to_string()];

  _write_tar(&jinx_service, &excluded, Some(jinx_files.jinx_home));
}

pub fn get_jinx_loadbalancer_tar() -> Vec<u8> {
  // get service
  let jinx_service = get_jinx_loadbalancer_service();

  get_tar(&jinx_service)
}

// returns a Vec<u8> of the tar.gz file
pub fn get_tar(jinx_service: &JinxService) -> Vec<u8> {
  // get jinx files
  let jinx_files = get_jinx_files();

  // create path
  let tar_file_path = format!(
    "{}/{}.jinx.tar.gz",
    jinx_files.jinx_home, &jinx_service.name
  );

  // open tar file
  let mut tar_file = File::open(&tar_file_path).expect("[TARGZ] Failed to open tar file");
  let mut tar_buffer = vec![];

  // read tar file
  tar_file
    .read_to_end(&mut tar_buffer)
    .expect("[TARGZ] Failed to read tar file");

  tar_buffer
}

// creates a tar of the project
pub fn write_tar(jinx_service: &JinxService, excluded: &Vec<String>, directory: Option<String>) {
  _write_tar(&jinx_service, &excluded, directory);
}

fn _write_tar(jinx_service: &JinxService, excluded: &Vec<String>, directory: Option<String>) {
  // get current directory
  let mut dir = env::current_dir().expect("[TARGZ] Failed to get current directory");

  // or use provided directory
  if directory.is_some() {
    dir = PathBuf::from(directory.expect("Failed to get directory"));
  }

  // get jinx files
  let jinx_files = get_jinx_files();

  // create paths
  let tar_file_path = format!(
    "{}/{}.jinx.tar.gz",
    jinx_files.jinx_home, &jinx_service.name
  );

  // create files
  let tar_file = File::create(&tar_file_path).expect("[TARGZ] Failed to create tar file");
  let mut tar_builder = Builder::new(tar_file);

  // get files in directory
  let paths = fs::read_dir(&dir).expect("Failed to read jinx directory");

  // iterate over files
  for path in paths {
    let p = path.expect("Failed to get path");
    let file_path = p.path();

    let file_name = match p.file_name().into_string() {
      Err(_) => log_exit!("[TARGZ] Failed to convert file name"),
      Ok(name) => name,
    };

    // check excluded list
    for ex in excluded.iter() {
      if file_name.contains(ex) {
        continue;
      }
    }

    // get file metadata
    let meta = match p.metadata() {
      Err(_err) => log_exit!("[TARGZ] Failed to get file metadata"),
      Ok(meta) => meta,
    };

    // check for directory or file
    if meta.is_dir() {
      tar_builder
        .append_dir_all(&file_name, &file_name)
        .expect("[TARGZ] Failed to append to tar file");
    } else {
      tar_builder
        .append_path_with_name(&file_path, &file_name)
        .expect("TARGZ] Failed to append to tar file");
    }
  }
}
