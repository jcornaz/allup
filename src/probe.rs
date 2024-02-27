use std::time::{Duration, Instant};

use reqwest::{Client, StatusCode};
use thiserror::Error;

use crate::config::Endpoint;

pub type Result = std::result::Result<Duration, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("timed out")]
    Timeout,
    #[error("responded {status}")]
    UnexpectedResponse {
        status: StatusCode,
        body: Option<String>,
    },
    #[error("network error: {0}")]
    Network(reqwest::Error),
}

pub async fn probe(endpoint: &Endpoint) -> Result {
    let req = Client::new()
        .get(endpoint.url.clone())
        .timeout(endpoint.timeout);

    let started = Instant::now();
    let resp = req.send().await;
    let duration = started.elapsed();

    let resp = resp.map_err(|err| {
        if err.is_timeout() {
            Error::Timeout
        } else {
            Error::Network(err)
        }
    })?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.ok();
        return Err(Error::UnexpectedResponse { status, body });
    }
    Ok(duration)
}
