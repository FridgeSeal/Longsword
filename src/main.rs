use anyhow::Result;
use log;
use models::{Document, Index};
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

mod dataio;
mod models;
mod pipeline;
mod settings;

use settings::Settings;

fn ingest_document(raw: String) -> Document {
    Document::new("test document", raw)
}

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed)?;
    let cfg = Settings::new()?;
    log::info!("Setup Config");
    let data = dataio::index_dir(&cfg.read_path)?;
    log::info!("Read data");
    let doc_set: Vec<Document> = data
        .iter()
        .map(|doc| ingest_document(doc.clone()))
        .collect();
    log::info!("Docs parsed");
    let index = Index::new("test index", doc_set);
    let search_results = index.search(&cfg.search_text)?;
    dbg!(&search_results.len(), &search_results);
    Ok(())
    // dbg!(&search_results);
}
