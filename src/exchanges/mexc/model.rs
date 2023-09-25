use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerTimeMEXC {
    #[serde(rename = "serverTime")]
    pub server_time: i64,
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryWithSignature<T> {
    #[serde(flatten)]
    pub query: T,
    pub signature: String,
}
impl<T> QueryWithSignature<T> {
    pub fn new(query: T, signature: String) -> Self {
        Self { query, signature }
    }
}
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllSymbolsMEXC {
    pub code: i32,
    pub data: Vec<String>,
    pub msg: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerPriceMEXC {
    pub symbol: String,
    pub price: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthMEXC {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbols {
    pub symbol: String,
    pub status: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: i64,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    #[serde(rename = "quotePrecision")]
    pub quote_precision: i64,
    #[serde(rename = "quoteAssetPrecision")]
    pub quote_asset_precision: i64,
    #[serde(rename = "baseCommissionPrecision")]
    pub base_commission_precision: i64,
    #[serde(rename = "quoteCommissionPrecision")]
    pub quote_commission_precision: i64,
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,
    //  #[serde(rename = "quoteOrderQtyMarketAllowed")]
    // pub quote_order_qty_market_allowed: bool,
    #[serde(rename = "isSpotTradingAllowed")]
    pub is_spot_trading_allowed: bool,
    #[serde(rename = "isMarginTradingAllowed")]
    pub is_margin_trading_allowed: bool,
    #[serde(rename = "quoteAmountPrecision")]
    pub quote_amount_precision: String,
    #[serde(rename = "baseSizePrecision")]
    pub base_size_precision: String,
    pub permissions: Vec<String>,
    pub filters: Vec<serde_json::Value>,
    #[serde(rename = "maxQuoteAmount")]
    pub max_quote_amount: String,
    #[serde(rename = "makerCommission")]
    pub maker_commission: String,
    #[serde(rename = "takerCommission")]
    pub taker_commission: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfoMEXC {
    pub timezone: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<serde_json::Value>,
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<serde_json::Value>,
    pub symbols: Vec<Symbols>,
}
