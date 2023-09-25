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
#[serde(rename_all = "camelCase")]
pub struct nil {}
#[derive(Debug)]
pub struct AllSymbols {
    pub symbols: Vec<String>,
}

#[derive(Debug)]
pub struct TickerPrice {
    pub symbol: String,
    pub price: String,
}
