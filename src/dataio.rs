use std::{fs, io};

pub fn read_data(fpath: &str) -> String {
    let data = fs::read_to_string(fpath);
    data.expect("Couldn't read file from path")
}

pub index_dir(dir_path: &str) -> io::Result<Vec<String>> {
    unimplemented!() // TODO
}
