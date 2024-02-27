use std::fs;

use clap::Parser;
use cli::Opts;
use config::Config;
use main_error::MainResult;

use tokio::task;

mod cli;
mod config;
mod probe;

#[tokio::main]
async fn main() -> MainResult {
    let opts = Opts::parse();
    let config: Config = toml::from_str(&fs::read_to_string(opts.file)?)?;
    let tasks = config.endpoints.into_iter().map(move |e| {
        task::spawn(async move {
            let res = probe::probe(&e).await;
            (e, res)
        })
    });
    for task in tasks {
        let (endpoint, result) = task.await?;
        println!("{}: {:?}", endpoint.name, result);
    }
    Ok(())
}
