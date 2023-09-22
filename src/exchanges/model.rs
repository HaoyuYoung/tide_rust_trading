pub struct Config {
    pub api_key: String,
    pub secret_key: String,
    pub passphrase: String,
    pub url: String,
}

#[derive(Debug)]
pub struct ServerTime {
    pub server_time: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct nil {}
