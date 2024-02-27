use std::time::Duration;

use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub url: Url,
    #[serde(default = "default_timeout")]
    pub timeout: Duration,
}

fn default_timeout() -> Duration {
    Duration::from_secs(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_example() {
        let config: Config = toml::from_str(include_str!("../example.toml")).unwrap();
        assert!(!config.endpoints.is_empty())
    }
}
