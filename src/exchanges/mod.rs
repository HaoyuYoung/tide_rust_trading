use crate::exchanges::model::{
    AllSymbols, Balance, Config, Depth, Kline, KlineInterval, ServerTime, SymbolInfo, TickerPrice,
    TradingFee,
};

pub mod mexc;
mod model;

pub trait Clients {
    fn new_client(&self, cfg: Config) -> Self;
    fn ping(&self) -> Result<bool, Box<dyn std::error::Error>>;
    fn time(&self) -> Result<ServerTime, Box<dyn std::error::Error>>;
}
pub trait Market {
    fn get_all_symbol(&self) -> Result<AllSymbols, Box<dyn std::error::Error>>;
    fn get_ticker_price(&self, symbol: String) -> Result<TickerPrice, Box<dyn std::error::Error>>;

    fn get_depth(&self, symbol: String, limit: String)
        -> Result<Depth, Box<dyn std::error::Error>>;
    fn get_symbol_info(&self, symbol: String) -> Result<SymbolInfo, Box<dyn std::error::Error>>;

    fn get_klines(
        &self,
        symbol: String,
        interval: KlineInterval,
        start_time: Option<u128>,
        end_time: Option<u128>,
        limit: i64,
    ) -> Result<Vec<Kline>, Box<dyn std::error::Error>>;
}

pub trait Account {
    fn get_trading_fee(&self, symbol: String) -> Result<TradingFee, Box<dyn std::error::Error>>;

    fn get_balance(&self, token: String) -> Result<Balance, Box<dyn std::error::Error>>;

    fn discount_status(&self) -> Result<bool, Box<dyn std::error::Error>>;
}
