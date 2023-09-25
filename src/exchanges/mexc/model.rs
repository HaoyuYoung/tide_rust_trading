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
