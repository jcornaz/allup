use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub url: Url,
    #[serde(default = "default_timeout")]
    pub timeout: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProbeResult {
    #[serde(flatten)]
    pub endpoint: Endpoint,
    pub duration: u128,
    pub error: Option<Error>,
}

#[derive(Debug, Clone, Serialize, Error)]
pub enum Error {
    #[error("timeout")]
    Timeout,
    #[error("status code: {0}")]
    UnexpectedStatusCode(u16),
    #[error("too many redirects")]
    TooManyRedirects,
    #[error("unreachable")]
    Unreachable,
}

fn default_timeout() -> u32 {
    10_000
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_parse_example() {
        let config: Config = toml::from_str(include_str!("../example.toml")).unwrap();
        assert!(!config.endpoints.is_empty())
    }

    #[test]
    fn should_serialize_results() {
        let result = vec![
            ProbeResult {
                endpoint: Endpoint {
                    name: "Google".into(),
                    url: "https://google.com".parse().unwrap(),
                    timeout: default_timeout(),
                },
                duration: 123,
                error: Some(Error::Timeout),
            },
            ProbeResult {
                endpoint: Endpoint {
                    name: "Google".into(),
                    url: "https://google.com".parse().unwrap(),
                    timeout: default_timeout(),
                },
                duration: 123,
                error: Some(Error::UnexpectedStatusCode(500)),
            },
        ];
        let _ = serde_json::to_string_pretty(&result).unwrap();
    }
}
