use anyhow::Result;
use mapr::MmapOptions;
use quick_xml::de::from_reader;
use std::fs::File;
use std::io::Cursor;

use crate::models::{Document, Feed};

pub fn load_data(fpath: &str) -> Result<Vec<Document>> {
    let texts: Vec<Document>;
    {
        log::info!("Starting load");
        let file = File::open(fpath)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        // MyFirstUnsafe lol
        let cursor = Cursor::new(mmap);
        log::info!("File memory-mapped, starting parse");
        let feed: Feed = from_reader(cursor)?;
        log::info!("Parsing complete");
        texts = feed.doc.into_iter().map(|f| f.into()).collect();
    }
    Ok(texts)
}

// pub fn index_dir(dir_path: &str) -> anyhow::Result<Vec<Document>> {
//     let mut file_data: Vec<Document> = Vec::with_capacity(1000);
//     let found = glob(dir_path)?.filter_map(Result::ok);
//     for file_path in found {
//         match fs::read_to_string(&file_path) {
//             Ok(data) => {
//                 let doc = Document::new(file_path.file_name().unwrap().to_str().unwrap(), data);
//                 file_data.push(doc);
//             }
//             Err(e) => {
//                 log::warn!(
//                     "Failed to read data for path: {}\nwith error: {}",
//                     file_path.display(),
//                     e
//                 );
//             }
//         };
//     }
//     file_data.shrink_to_fit();
//     Ok(file_data)
// }
