use anyhow::Result;
use log;
use models::Index;
use rkyv::ser::{serializers::WriteSerializer, Serializer};
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use std::fs;
use std::io::prelude::*;
mod dataio;
mod models;
mod pipeline;
mod settings;

use settings::Settings;

fn write_doc(data: Vec<u8>, path: &str) -> Result<()> {
    let mut output_file = fs::File::create(format!("{}.rkyv", path))?;
    output_file.write_all(&data)?;
    Ok(())
}

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed)?;
    let cfg = Settings::new()?;
    log::info!("Setup Config");
    let data = dataio::index_dir(&cfg.read_path)?;
    log::info!("Docs parsed");
    let index = Index::new("test index", data);
    let search_results = index.search(&cfg.search_text)?;
    // dbg!(&search_results.len(), &search_results);
    let first_key = index.keys.first().unwrap();
    let test_textdata = models::TextData {
        id: 47,
        name: index.documents.get(*first_key).unwrap().name.clone(),
        text: index
            .documents
            .get(*first_key)
            .unwrap()
            .sentence_set
            .clone(),
    };
    //
    let mut serializer = WriteSerializer::new(Vec::new());
    let _pos = serializer
        .serialize_value(&test_textdata)
        .expect("failed to archive test");
    let buf = serializer.into_inner();
    write_doc(buf, &test_textdata.name)?;
    dbg!(&search_results);
    Ok(())
}
