use crate::exchanges::mexc::model::{OrderResutl, TradingSymbolsMEXC};
use crate::exchanges::mexc::spot::client::Client;
use crate::exchanges::model::{OrderID, OrderType, Side};
use crate::exchanges::Trade;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

impl Client {
    fn get_side_mexc(side: Side) -> String {
        match side {
            Side::Ask => String::from("SELL"),
            Side::Bid => String::from("BUY"),
        }
    }
    fn get_order_type_mexc(order_type: OrderType) -> String {
        match order_type {
            OrderType::Limit => String::from("LIMIT"),
            OrderType::Market => String::from("MARKET"),
            OrderType::Maker => String::from("LIMIT_MAKER"),
            OrderType::Taker => String::from("IMMEDIATE_OR_CANCEL"),
            OrderType::FOK => String::from("FILL_OR_KILL"),
        }
    }
    fn get_side(side: String) -> Side {
        match side.as_str() {
            "SELL" => Side::Ask,
            "BUY" => Side::Bid,
            _ => Side::Ask,
        }
    }
    fn get_order_type(order_type: String) -> OrderType {
        match order_type.as_str() {
            "LIMIT" => OrderType::Limit,
            "MARKET" => OrderType::Market,
            "LIMIT_MAKER" => OrderType::Maker,
            "IMMEDIATE_OR_CANCEL" => OrderType::Taker,
            "FILL_OR_KILL" => OrderType::FOK,
            _ => OrderType::Limit,
        }
    }
}
impl Trade for Client {
    fn trading_symbols(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/selfSymbols"));
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

        let r: TradingSymbolsMEXC = res.json()?;
        println!("{:?} {:?}", "Full Msg:  ", &r);

        Ok(r.data)
    }

    fn order(
        &self,
        symbol: String,
        order_type: OrderType,
        side: Side,
        price: Option<String>,
        mut amount: Option<String>,
        quote_amount: Option<String>,
        hidden: bool,
    ) -> Result<OrderID, Box<dyn Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/order"));
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Paramters {
            symbol: String,
            side: String,
            #[serde(rename = "type")]
            order_type: String,
            quantity: Option<String>,
            quoteOrderQty: Option<String>,
            price: Option<String>,
            recv_window: Option<u64>,
            timestamp: Option<u128>,
        }
        if order_type != OrderType::Market || amount.is_none() || quote_amount.is_some() {
            amount = Option::from(
                (quote_amount.clone().unwrap().parse::<f64>().unwrap()
                    / price.clone().unwrap().parse::<f64>().unwrap())
                .to_string(),
            )
        }
        let p = Paramters {
            symbol: symbol.clone(),
            side: Client::get_side_mexc(side),
            order_type: Client::get_order_type_mexc(order_type),
            quantity: amount,
            quoteOrderQty: quote_amount,
            price,
            recv_window: None,
            timestamp: Option::from(time),
        };
        let res = self.post(url, p, true)?;
        println!("{:?}", res);
        let r: OrderResutl = res.json()?;
        println!("{:?} {:?}", "Full Msg:  ", &r);

        Ok(OrderID {
            symbol: symbol.clone(),
            order_id: r.order_id,
            transaction_time: r.transact_time as i128,
        })
    }
}
mod tests {
    use crate::exchanges::mexc::spot::client::Client;
    use crate::exchanges::model::{Config, OrderType, Side};
    use crate::exchanges::{Account, Market, Trade};
    use serde_json::to_string;

    #[test]
    fn time_test() {
        let cfg = Config {
            api_key: "".to_string(),
            secret_key: "".to_string(),
            passphrase: "".to_string(),
            url: "https://api.mexc.com".to_string(),
        };
        let c = Client::new_client_cfg(cfg);
        let r = c.order(
            "MXUSDT".to_string(),
            OrderType::Maker,
            Side::Ask,
            Option::from("2.5".to_string()),
            None,
            Option::from("5.5".to_string()),
            false,
        );
        println!("{:?}", r);
    }
}
