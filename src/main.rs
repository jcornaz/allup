use std::{fs, io::stdout, time::Duration};

use clap::Parser;
use cli::Opts;
use config::{Config, Endpoint};
use main_error::MainResult;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::{
    sync::mpsc,
    task::{self},
    time,
};
use tui::State;

mod cli;
mod config;
mod probe;
mod tui;

#[tokio::main]
async fn main() -> MainResult {
    let opts = Opts::parse();
    let config: Config = toml::from_str(&fs::read_to_string(opts.file)?)?;
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(stdout()),
        ratatui::TerminalOptions {
            viewport: ratatui::Viewport::Inline(config.endpoints.len() as u16 + 1),
        },
    )?;
    let mut state = State::new(&config.endpoints);
    let (sender, mut receiver) = mpsc::channel::<(usize, probe::Result)>(config.endpoints.len());
    let probes = tokio::spawn(send_probes(config.endpoints, sender));
    terminal.draw(|frame| state.view(frame))?;
    while !probes.is_finished() {
        time::sleep(Duration::from_millis(100)).await;
        while let Ok((i, r)) = receiver.try_recv() {
            state.set_result(i, r);
        }
        state.tick();
        terminal.draw(|frame| state.view(frame))?;
    }
    probes.await??;
    terminal.clear()?;
    drop(terminal);
    println!("Everything is UP {:?}", state.result()?);
    Ok(())
}

async fn send_probes(
    endpoints: impl IntoIterator<Item = Endpoint>,
    sender: mpsc::Sender<(usize, probe::Result)>,
) -> anyhow::Result<()> {
    let tasks: Vec<_> = endpoints
        .into_iter()
        .enumerate()
        .map(move |(i, e)| {
            let sender = sender.clone();
            task::spawn(async move {
                let res = probe::probe(&e).await;
                sender.send((i, res)).await
            })
        })
        .collect();
    for task in tasks {
        task.await??;
    }
    Ok(())
}
