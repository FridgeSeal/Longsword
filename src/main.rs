use anyhow::Result;
use log;
use models::Index;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

mod dataio;
mod models;
mod pipeline;
mod settings;

use settings::Settings;

}

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed)?;
    let cfg = Settings::new()?;
    log::info!("Setup Config");
    let data = dataio::index_dir(&cfg.read_path)?;
    log::info!("Docs parsed");
    let index = Index::new("test index", data);
    let search_results = index.search(&cfg.search_text)?;
    dbg!(&search_results.len(), &search_results);
    Ok(())
    // dbg!(&search_results);
}
