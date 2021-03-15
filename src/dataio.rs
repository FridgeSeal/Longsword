use crate::models::Document;
use anyhow;
use fs::read_to_string;
use glob::glob;
use log;
use std::fs;

pub fn index_dir(dir_path: &str) -> anyhow::Result<Vec<Document>> {
    let mut file_data: Vec<Document> = Vec::with_capacity(1000);
    let found = glob(dir_path)?.filter_map(Result::ok);
    for file_path in found {
        match read_to_string(&file_path) {
            Ok(data) => {
                let doc = Document::new(file_path.file_name().unwrap().to_str().unwrap(), data);
                file_data.push(doc);
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
