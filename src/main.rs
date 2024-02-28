use std::{
    fs,
    io::{self},
    process::{self},
};

use clap::Parser;
use cli::Opts;
use main_error::MainResult;
use model::Config;

use probe::probe;
use render::render;
use tokio::task::{self};

mod cli;
mod model;
mod probe;
mod render;

#[tokio::main]
async fn main() -> MainResult {
    let opts = Opts::parse();
    let config: Config = toml::from_str(&fs::read_to_string(opts.file)?)?;
    let probes: Vec<_> = config
        .endpoints
        .into_iter()
        .map(|e| task::spawn(probe(e)))
        .collect();
    let mut results = Vec::with_capacity(probes.len());
    for probe in probes {
        results.push(probe.await?);
    }
    if opts.json {
        serde_json::to_writer_pretty(io::stdout(), &results)?;
    } else {
        render(io::stdout(), &results)?;
    }
    if results.iter().any(|r| r.error.is_some()) {
        process::exit(1);
    }
    Ok(())
}
