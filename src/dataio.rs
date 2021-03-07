use std::{fs, io};

use anyhow;
use fs::read_to_string;
use glob::glob;

pub fn read_data(fpath: &str) -> String {
    let data = fs::read_to_string(fpath);
    data.expect("Couldn't read file from path")
}

pub fn index_dir(dir_path: &str) -> anyhow::Result<Vec<String>> {
    let mut file_data: Vec<String> = Vec::with_capacity(1000);
    for file_path in glob(dir_path)?.filter_map(Result::ok) {
        match read_to_string(&file_path) {
            Ok(data) => {
                file_data.push(data);
            }
            Err(e) => {
                log::warn!(
                    "Failed to read data for path: {}\nwith error: {}",
                    file_path.display(),
                    e
                );
            }
        };
    }
    file_data.shrink_to_fit();
    Ok(file_data)
}
