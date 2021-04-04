use std::io::{self};

use anyhow::Result;
use log;
use models::Index;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
mod dataio;
mod models;
mod settings;
use settings::Settings;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )?;
    let cfg = Settings::new()?;
    log::info!("Setup Config");
    let docs = dataio::load_data(&cfg.read_path)?;
    log::info!("n docs: {}", docs.len());
    let mut idx = models::Index::new();
    log::info!("Starting doc indexing");
    docs.into_iter().for_each(|d| {
        idx.insert(d).expect("Couldn't insert document");
    });
    log::info!("Finished doc indexing");
    // input_loop(&idx)?;
    search_test(&idx);
    Ok(())
}

fn search_test(idx: &Index) {
    let searches = [
        "Articolo 31 is a band from Milan, Italy",
        "financial",
        "dogs",
        "logarithmic astronomical magnitude scale",
        "antiderivative",
        "Rwanda, and Burundi",
    ];
    for search in searches.iter() {
        idx.search(search);
    }
}

fn input_loop(index: &Index) -> anyhow::Result<()> {
    let mut input = "".to_string();
    while input != "q" {
        input = input_handler()?;
        log::info!("Search term: {}", input);
        let results = index.search(&input);
        log::info!("Results: {}", results);
        println!("");
    }
    Ok(())
}

fn input_handler() -> Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input");
    log::info!("Captured input: {}", buffer);
    Ok(buffer)
}
