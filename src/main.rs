use std::{fs, process::exit};

use clap::Parser;
use cli::Opts;
use config::{Config, Endpoint};
use main_error::MainResult;

use tokio::{
    sync::mpsc,
    task::{self},
};

mod cli;
mod config;
mod probe;

#[tokio::main]
async fn main() -> MainResult {
    let opts = Opts::parse();
    let config: Config = toml::from_str(&fs::read_to_string(opts.file)?)?;
    let (sender, mut receiver) = mpsc::channel::<(Endpoint, probe::Result)>(config.endpoints.len());
    let probes = tokio::spawn(send_probes(config.endpoints, sender));
    let mut error = false;
    while let Some((endpoint, result)) = receiver.recv().await {
        print!("{}: ", endpoint.name);
        match result {
            Ok(duration) => println!("OK {duration:?}"),
            Err(err) => {
                error = true;
                println!("ERROR: {err}");
            }
        }
    }
    probes.await??;
    if error {
        exit(1);
    }
    Ok(())
}

async fn send_probes(
    endpoints: impl IntoIterator<Item = Endpoint>,
    sender: mpsc::Sender<(Endpoint, probe::Result)>,
) -> anyhow::Result<()> {
    let tasks: Vec<_> = endpoints
        .into_iter()
        .map(move |e| {
            let sender = sender.clone();
            task::spawn(async move {
                let res = probe::probe(&e).await;
                sender.send((e, res)).await
            })
        })
        .collect();
    for task in tasks {
        task.await??;
    }
    Ok(())
}
