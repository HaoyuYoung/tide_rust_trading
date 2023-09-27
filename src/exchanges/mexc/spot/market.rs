use crate::exchanges::mexc::model::{
    AllSymbolsMEXC, DepthMEXC, KlineMEXC, SymbolInfoMEXC, TickerPriceMEXC,
};
use crate::exchanges::mexc::spot::client::Client;
use crate::exchanges::model::{
    AllSymbols, Depth, Kline, KlineInterval, PriceLevel, SymbolInfo, TickerPrice,
};
use crate::exchanges::Market;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

impl Client {
    fn get_interval_mexc(interval: KlineInterval) -> String {
        match interval {
            KlineInterval::Min1 => String::from("1m"),
            KlineInterval::Min5 => String::from("5m"),
            KlineInterval::Min15 => String::from("15m"),
            KlineInterval::Min30 => String::from("30m"),
            KlineInterval::H1 => String::from("60m"),
            KlineInterval::H4 => String::from("4h"),
            KlineInterval::D1 => String::from("1d"),
            KlineInterval::M1 => String::from("1M"),
            _ => String::from("1d"),
        }
    }
}
impl Market for Client {
    fn get_all_symbol(&self) -> Result<AllSymbols, Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/defaultSymbols"));
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Paramters {
            recv_window: Option<u64>,
            timestamp: Option<u128>,
        }
        let p = Paramters {
            recv_window: None,
            timestamp: Option::from(time),
        };
        let res = self.get(url, p, false)?;

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
        struct Paramters {
            symbol: String,
            recv_window: Option<u64>,
            timestamp: Option<u128>,
        }
        let p = Paramters {
            symbol,
            recv_window: None,
            timestamp: Option::from(time),
        };
        let res = self.get(url, p, false)?;

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
        struct Paramters {
            symbol: String,
            limit: Option<i64>,
            recv_window: Option<u64>,
            timestamp: Option<u128>,
        }
        let p = Paramters {
            symbol,
            limit: Option::from(limit.parse::<i64>().unwrap()),
            recv_window: None,
            timestamp: Option::from(time),
        };
        let res = self.get(url, p, false)?;

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
        struct Paramters {
            symbol: Option<String>,

            recv_window: Option<u64>,
            timestamp: Option<u128>,
        }
        let p = Paramters {
            symbol: Option::from(symbol),

            recv_window: None,
            timestamp: Option::from(time),
        };
        let res = self.get(url, p, false)?;

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

    fn get_klines(
        &self,
        symbol: String,
        interval: KlineInterval,
        start_time: Option<u128>,
        end_time: Option<u128>,
        limit: i64,
    ) -> Result<Vec<Kline>, Box<dyn Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/klines"));
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let interval_mexc = Client::get_interval_mexc(interval);
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Paramters {
            symbol: String,
            interval: String,
            startTime: Option<u128>,
            endTime: Option<u128>,
            limit: i64,
            recv_window: Option<u64>,
            timestamp: Option<u128>,
        }
        let p = Paramters {
            symbol: symbol.clone(),
            interval: interval_mexc,
            startTime: start_time,
            endTime: end_time,
            limit,
            recv_window: None,
            timestamp: Option::from(time),
        };
        let res = self.get(url, p, false)?;

        let r: Vec<KlineMEXC> = res.json()?;
        let mut klines: Vec<Kline> = Vec::new();

        for kline in r {
            let kl = Kline {
                symbol: symbol.clone(),
                interval,
                open_time: kline.open_time,
                close_time: kline.close_time,
                open: kline.open,
                high: kline.high,
                low: kline.low,
                close: kline.close,
                volume: kline.volume,
                volume_quote: kline.quote_asset_volume,
            };
            klines.push(kl);
        }

        Ok(klines)
    }
}
mod tests {

    use crate::exchanges::mexc::spot::client::Client;
    use crate::exchanges::model::{Config, KlineInterval};
    use crate::exchanges::Market;

    #[test]
    fn time_test() {
        let cfg = Config {
            api_key: "".to_string(),
            secret_key: "".to_string(),
            passphrase: "".to_string(),
            url: "https://api.mexc.com".to_string(),
        };
        let c = Client::new_client_cfg(cfg);
        let r = c.get_klines("BTCUSDT".to_string(), KlineInterval::Min1, None, None, 10);
        println!("{:?}", r);
    }
}
