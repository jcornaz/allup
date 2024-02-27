use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
#[allow(unused)]
pub struct Config {
    endpoints: Vec<Endpoint>,
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct Endpoint {
    url: Url,
}
