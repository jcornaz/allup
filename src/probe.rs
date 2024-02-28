use std::time::{Duration, Instant};

use reqwest::{Client, Response, StatusCode};
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

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::Timeout
        } else {
            Error::Network(err)
        }
    }
}

pub async fn probe(endpoint: &Endpoint) -> Result {
    let req = request(endpoint);
    let (resp, duration) = send(req).await?;
    ensure_success(resp).await?;
    Ok(duration)
}

fn request(endpoint: &Endpoint) -> reqwest::RequestBuilder {
    Client::new()
        .get(endpoint.url.clone())
        .timeout(endpoint.timeout)
}

async fn send(req: reqwest::RequestBuilder) -> reqwest::Result<(reqwest::Response, Duration)> {
    let started = Instant::now();
    let resp = req.send().await?;
    let duration = started.elapsed();
    Ok((resp, duration))
}

async fn ensure_success(resp: Response) -> std::result::Result<(), Error> {
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.ok();
        return Err(Error::UnexpectedResponse { status, body });
    }
    Ok(())
}
