use log;
use models::Document;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

mod dataio;
mod models;
mod pipeline;
mod settings;

use settings::Settings;

fn ingest_document(raw: String) -> Document {
    Document::new("test document", raw)
}

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap();
    let cfg = Settings::new().unwrap();
    log::info!("Setup Config");
    let data = dataio::read_data(&cfg.read_path);
    log::info!("Read data");
    let index = ingest_document(data);
    log::info!("Index built");
    let search_results = index.search(&cfg.search_text);
    dbg!(&search_results.len());
    // dbg!(&search_results);
}
