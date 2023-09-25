use crate::exchanges::mexc::model::{AllSymbolsMEXC, DepthMEXC, SymbolInfoMEXC, TickerPriceMEXC};
use crate::exchanges::mexc::spot::client::Client;
use crate::exchanges::model::{AllSymbols, Depth, PriceLevel, SymbolInfo, TickerPrice};
use crate::exchanges::Market;
use std::error::Error;
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

    fn get_depth(&self, symbol: String, limit: String) -> Result<Depth, Box<dyn Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/depth"));
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Paramters {
            symbol: Option<String>,
            limit: Option<i64>,
            recv_window: Option<u64>,
            timestamp: Option<u128>,
        }
        let p = Paramters {
            symbol: Option::from(symbol),
            limit: Option::from(limit.parse::<i64>().unwrap()),
            recv_window: Option::from(5000),
            timestamp: Option::from(time),
        };
        let res = self.get(url, p)?;

        let r: DepthMEXC = res.json()?;

        let mut d = Depth {
            bids: vec![],
            asks: vec![],
        };
        for (i, pl) in r.asks.iter().enumerate() {
            d.asks.push(PriceLevel {
                price: pl[0].clone(),
                size: pl[1].clone(),
            });
            d.bids.push(PriceLevel {
                price: r.bids[i][0].clone(),
                size: r.bids[i][1].clone(),
            });
        }
        Ok(d)
    }

    fn get_symbol_info(&self, symbol: String) -> Result<SymbolInfo, Box<dyn Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/exchangeInfo"));
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

        let r: SymbolInfoMEXC = res.json()?;
        println!("{:?} {:?}", "Full Msg:  ", &r);
        Ok(SymbolInfo {
            symbol: r.symbols[0].symbol.clone(),
            base_precision: r.symbols[0].base_asset_precision.clone(),
            quote_precision: r.symbols[0].quote_asset_precision.clone(),
            min_quote_amount: r.symbols[0].quote_amount_precision.clone(),
            min_base_amount: r.symbols[0].base_size_precision.clone(),
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
        let r = c.get_symbol_info("NAOUSDT".to_string());
        println!("{:?}", r)
    }
}
