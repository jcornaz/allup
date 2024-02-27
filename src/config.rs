use std::time::Duration;

use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Config {
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Endpoint {
    pub name: String,
    pub url: Url,
    pub follow_redirect: bool,
    #[serde(default = "default_timeout")]
    pub timeout: Duration,
}

fn default_timeout() -> Duration {
    Duration::from_secs(10)
}
