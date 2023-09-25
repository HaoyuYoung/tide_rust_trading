use crate::exchanges::mexc::model::{AllSymbolsMEXC, TickerPriceMEXC};
use crate::exchanges::mexc::spot::client::Client;
use crate::exchanges::model::{AllSymbols, TickerPrice};
use crate::exchanges::Market;
use std::time::{SystemTime, UNIX_EPOCH};

impl Market for Client {
    fn get_all_symbol(&self) -> Result<AllSymbols, Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/defaultSymbols"));
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Paramters {
            recv_window: Option<u64>,
            timestamp: Option<u128>,
        }
        let p = Paramters {
            recv_window: Option::from(5000),
            timestamp: Option::from(time),
        };
        let res = self.get(url, p)?;

        let r: AllSymbolsMEXC = res.json()?;

        Ok(AllSymbols { symbols: r.data })
    }

    fn get_ticker_price(&self, symbol: String) -> Result<TickerPrice, Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/ticker/price"));
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Paramters {
            symbol: Option<String>,
            recv_window: Option<u64>,
            timestamp: Option<u128>,
        }
        let p = Paramters {
            symbol: Option::from(symbol),
            recv_window: Option::from(5000),
            timestamp: Option::from(time),
        };
        let res = self.get(url, p)?;

        let r: TickerPriceMEXC = res.json()?;

        Ok(TickerPrice {
            symbol: r.symbol,
            price: r.price,
        })
    }
}
mod tests {
    use crate::exchanges::mexc::spot::client::new_client_cfg;
    use crate::exchanges::model::Config;
    use crate::exchanges::Market;

    #[test]
    fn time_test() {
        let cfg = Config {
            api_key: "".to_string(),
            secret_key: "".to_string(),
            passphrase: "".to_string(),
            url: "https://api.mexc.com".to_string(),
        };
        let c = new_client_cfg(cfg);
        let r = c.get_all_symbol();
        println!("{:?}", r.unwrap().symbols[0])
    }
}
