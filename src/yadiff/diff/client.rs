use std::{collections::HashMap, fs};

use hex::encode;
use ring::digest;

use crate::filescanner;
use crate::yadisk;

use super::File;

pub fn from_yadisk(resources: Vec<yadisk::Resource>, parent: &String) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    let parent = "disk:/".to_string() + parent;

    for resource in resources {
        let file = File {
            path: str::replace(&resource.path, &parent, ""),
            sha256: resource.sha256.unwrap(),
            size: resource.size.unwrap(),
        };
        files.push(file);
    }

    files
}

pub fn from_local(filepaths: Vec<filescanner::Resource>, parent: &String) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    for path in filepaths {
        let content = fs::read(&path.path).unwrap();

        let file = File {
            path: str::replace(&path.path, parent, ""),
            sha256: encode(digest::digest(&digest::SHA256, &content)),
            size: fs::metadata(path.path).unwrap().len(),
        };
        files.push(file);
    }

    files
}

pub fn find_diff(yadisk: &Vec<File>, local: &Vec<File>) {
    let mut yadisk_map = HashMap::new();
    for file in yadisk.iter() {
        yadisk_map.insert(&file.sha256, file);
    }

    let mut local_map = HashMap::new();
    for file in local {
        local_map.insert(&file.sha256, file);
    }

    for file in yadisk.iter() {
        match local_map.get(&file.sha256) {
            None => println!("not found locally: {}", file.path),
            Some(&localfile) => println!("equal: {} == {}", file.path, localfile.path),
        }
    }

    for file in local.iter() {
        match yadisk_map.get(&file.sha256) {
            None => println!("not found on  yandex: {}", file.path),
            _ => (),
        }
    }
}
