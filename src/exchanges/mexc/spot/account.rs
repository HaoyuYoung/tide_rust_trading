use crate::exchanges::mexc::model::{AccountInfo, DiscountMEXC};
use crate::exchanges::mexc::spot::client::Client;
use crate::exchanges::model::{Balance, TradingFee};
use crate::exchanges::Account;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

impl Account for Client {
    fn get_trading_fee(&self, symbol: String) -> Result<TradingFee, Box<dyn Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/account"));
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
        let res = self.get(url, p, true)?;

        let r: AccountInfo = res.json()?;
        println!("{:?} {:?}", "Full Msg:  ", &r);
        Ok(TradingFee {
            maker: (r.maker_commission / 10000).to_string(),
            taker: (r.taker_commission / 10000).to_string(),
        })
    }

    fn get_balance(&self, token: String) -> Result<Balance, Box<dyn Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/account"));
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
        let res = self.get(url, p, true)?;

        let r: AccountInfo = res.json()?;
        println!("{:?} {:?}", "Full Msg:  ", &r);
        let mut balance = Balance {
            token: "".to_string(),
            free: "".to_string(),
            locked: "".to_string(),
            total: "".to_string(),
        };
        for tokens in r.balances {
            if token == tokens.asset {
                balance.token = tokens.asset;
                balance.free = tokens.free.clone();
                balance.locked = tokens.locked.clone();
                balance.total = (tokens.free.parse::<f64>().unwrap()
                    + tokens.locked.parse::<f64>().unwrap())
                .to_string();
                break;
            }
        }
        Ok(balance)
    }

    fn discount_status(&self) -> Result<bool, Box<dyn Error>> {
        let url: String = format!("{}{}", self.url, String::from("api/v3/mxDeduct/enable"));
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
            recv_window: Option::from(500),
            timestamp: Option::from(time),
        };
        let res = self.get(url, p, true)?;

        let r: DiscountMEXC = res.json()?;
        println!("{:?} {:?}", "Full Msg:  ", &r);

        Ok(r.data.mx_deduct_enable)
    }
}

mod tests {
    use crate::exchanges::mexc::spot::client::new_client_cfg;
    use crate::exchanges::model::Config;
    use crate::exchanges::{Account, Market};

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
        println!("{:?}", r);
    }
}
