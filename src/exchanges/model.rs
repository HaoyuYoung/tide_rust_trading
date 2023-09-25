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
#[derive(Debug)]

pub struct Depth {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}
#[derive(Debug)]
pub struct PriceLevel {
    pub price: String,
    pub size: String,
}
#[derive(Debug)]
pub struct SymbolInfo {
    pub symbol: String,
    pub base_precision: i64,
    pub quote_precision: i64,
    pub min_quote_amount: String,
    pub min_base_amount: String,
}
