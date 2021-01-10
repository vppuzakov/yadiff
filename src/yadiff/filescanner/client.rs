use walkdir::WalkDir;

use super::Categories;
use super::Resource;

pub fn fixnames(files: Vec<Resource>, parent: &String) {
    for mut file in files {
        file.fixpath(parent)
    }
}

pub fn get_all_files(path: &String) -> Vec<Resource> {
    let mut files: Vec<Resource> = Vec::new();

    for entry in WalkDir::new(path) {
        let filepath = entry.unwrap();

        if !filepath.file_type().is_file() {
            continue;
        }

        let file = Resource {
            name: filepath.file_name().to_owned().into_string().unwrap(),
            path: str::replace(
                &filepath
                    .path()
                    .as_os_str()
                    .to_owned()
                    .into_string()
                    .unwrap(),
                path,
                "",
            ),
            category: Categories::FILE,
        };
        files.push(file);
    }

    files
}
