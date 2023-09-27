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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineMEXC {
    pub open_time: u128,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,

    pub close_time: u128,
    pub quote_asset_volume: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceMEXC {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    #[serde(rename = "makerCommission")]
    pub maker_commission: i64,
    #[serde(rename = "takerCommission")]
    pub taker_commission: i64,
    #[serde(rename = "buyerCommission")]
    pub buyer_commission: i64,
    #[serde(rename = "sellerCommission")]
    pub seller_commission: i64,
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    #[serde(rename = "updateTime")]
    pub update_time: Option<i128>,
    #[serde(rename = "accountType")]
    pub account_type: String,
    pub balances: Vec<BalanceMEXC>,
    pub permissions: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "mxDeductEnable")]
    pub mx_deduct_enable: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountMEXC {
    pub data: Data,
    pub code: i64,
    pub msg: String,
    pub timestamp: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingSymbolsMEXC {
    pub code: i64,
    pub data: Vec<String>,
    pub msg: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResutl {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    pub price: String,
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub side: String,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
}
