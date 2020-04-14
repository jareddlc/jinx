use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use tar::Builder;

use super::log_exit;
use crate::jinx;
use crate::jinx::JinxService;

pub fn create_tar(jinx_service: &JinxService, excluded: &Vec<String>) {
  // get current directory
  let current_dir = env::current_dir().expect("[TARGZ] Failed to get current directory");

  // get jinx directories
  let jinx_directories = jinx::get_jinx_directories();

  // get jinx service name
  let file_name = match &jinx_service.name {
    None => log_exit!("[TARGZ] Failed to load jinx.json"),
    Some(name) => name,
  };

  // create paths
  let tar_file_path = format!("{}/{}.tar.gz", jinx_directories.jinx_dir, file_name);

  // create files
  let tar_file = File::create(&tar_file_path).expect("[TARGZ] Failed to create tar file");
  let mut tar_builder = Builder::new(tar_file);

  // add files to targz file
  let _entries = fs::read_dir(current_dir)
    .expect("[TARGZ] Failed to create read directory")
    .map(|res| {
      res.map(|e| {
        let file_name = match e.file_name().into_string() {
          Err(_err) => "jinx_non_existant".to_string(),
          Ok(name) => name,
        };

        // check excluded list
        if excluded.contains(&file_name) {
          return e.path();
        }

        // get file metadata
        let meta = match e.metadata() {
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
            .append_path(&file_name)
            .expect("[TARGZ] Failed to append to tar file");
        }

        e.path()
      })
    })
    .collect::<Result<Vec<_>, io::Error>>()
    .expect("[TARGZ] Failed to collect directory");
}

pub fn get_tar(jinx_service: &JinxService) -> Vec<u8> {
  // get jinx directories
  let jinx_directories = jinx::get_jinx_directories();

  // get jinx service name
  let file_name = match &jinx_service.name {
    None => log_exit!("[TARGZ] Failed to load jinx.json"),
    Some(name) => name,
  };

  // create path
  let tar_file_path = format!("{}/{}.tar.gz", jinx_directories.jinx_dir, file_name);

  // open tar file
  let mut tar_file = File::open(&tar_file_path).expect("[TARGZ] Failed to open tar file");
  let mut tar_buffer = vec![];

  // read tar file
  tar_file
    .read_to_end(&mut tar_buffer)
    .expect("[TARGZ] Failed to read tar file");

  tar_buffer
}
