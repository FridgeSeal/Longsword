use std::fs;

pub fn read_data(fpath: &str) -> String {
    let data = fs::read_to_string(fpath);
    data.expect("Couldn't read file from path")
}
