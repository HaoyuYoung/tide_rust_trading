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
#[derive(Debug)]
pub struct Kline {
    pub symbol: String,
    pub interval: KlineInterval,
    pub open_time: u128,
    pub close_time: u128,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub volume_quote: String,
}
#[derive(Debug, Clone, Copy)]
pub enum KlineInterval {
    Min1,
    Min5,
    Min15,
    Min30,
    H1,
    H4,
    H8,
    D1,
    W1,
    M1,
}
#[derive(Debug)]
pub struct TradingFee {
    pub maker: String,
    pub taker: String,
}
#[derive(Debug)]
pub struct Balance {
    pub token: String,
    pub free: String,
    pub locked: String,
    pub total: String,
}
#[derive(Debug, Clone, Copy)]
pub enum Side {
    Ask,
    Bid,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
    Taker,
    Maker,
    FOK,
}
#[derive(Debug)]
pub struct OrderID {
    pub symbol: String,
    pub order_id: String,
    pub transaction_time: i128,
}
