use std::time::{Duration, Instant};

use reqwest::Client;

use crate::model::{Endpoint, Error, ProbeResult};

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::Timeout
        } else if err.is_redirect() {
            Error::TooManyRedirects
        } else if let Some(status) = err.status() {
            Error::UnexpectedStatusCode(status.as_u16())
        } else {
            Error::Unreachable
        }
    }
}

pub async fn probe(endpoint: Endpoint) -> ProbeResult {
    let req = request(&endpoint);
    let start_time = Instant::now();
    let resp = req.send().await;
    let duration = start_time.elapsed().as_millis();
    let error = resp
        .and_then(|r| r.error_for_status())
        .err()
        .map(Error::from);
    ProbeResult {
        endpoint,
        duration,
        error,
    }
}

fn request(endpoint: &Endpoint) -> reqwest::RequestBuilder {
    Client::new()
        .get(endpoint.url.clone())
        .timeout(Duration::from_millis(endpoint.timeout.into()))
}
