use crate::exchanges::mexc::model::{QueryWithSignature, ServerTimeMEXC};
use crate::exchanges::model::{nil, Config, ServerTime};
use crate::exchanges::Exchanges;
use std::io::Error;

use hmac::{Hmac, Mac};
use http::{HeaderMap, HeaderValue};
use reqwest::blocking::Response;

use sha2::Sha256;

pub struct Client {
    api_key: String,
    secret_key: String,
    passphrase: String,
    url: String,
    client: reqwest::blocking::Client,
}

impl Client {
    fn get<T>(&self, url: String, query: T) -> Result<Response, Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        let mut headers = HeaderMap::new();

        headers.insert(
            "X-MEXC-APIKEY",
            HeaderValue::from_str(self.api_key.as_str())?,
        );
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        let signed = self.sign_query(query)?;
        let res = self
            .client
            .get(url.as_str())
            .query(&signed)
            .headers(headers)
            .send()?;
        match res.status().as_str() {
            "200" => Ok(res),
            _ => Err(Box::from(res.text()?.as_str())),
        }
    }
    fn sign_query<T>(&self, query: T) -> Result<QueryWithSignature<T>, Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        let query_string = serde_urlencoded::to_string(&query)?;
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes())?;
        mac.update(query_string.as_bytes());
        let mac_result = mac.finalize();
        let mac_bytes = mac_result.into_bytes();
        let signature = hex::encode(mac_bytes);

        Ok(QueryWithSignature::new(query, signature))
    }
}

impl Exchanges for Client {
    fn new_client(&self, c: Config) -> Self {
        Client {
            api_key: c.api_key,
            secret_key: c.secret_key,
            passphrase: c.passphrase,
            url: c.url,
            client: reqwest::blocking::Client::new(),
        }
    }

    fn ping(&self) -> Result<(bool), Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/ping"));
        let res = self.get(url, nil {})?;
        Ok(res.status().is_success())
    }
    fn time(&self) -> Result<(ServerTime), Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/time"));
        let res = self.get(url, nil {})?;
        res.status().is_success();
        let r: ServerTimeMEXC = res.json()?;
        Ok(ServerTime {
            server_time: r.server_time,
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::exchanges::mexc::spot::client::Client;
    use crate::exchanges::model::Config;
    use crate::exchanges::Exchanges;
    use std::collections::HashMap;

    #[test]
    fn time_test() {
        let mut c: Client = Client {
            api_key: "".to_string(),
            secret_key: "".to_string(),
            passphrase: "".to_string(),
            url: "".to_string(),
            client: Default::default(),
        };
        let cfg = Config {
            api_key: "mx0vglNppksUeR3yhy".to_string(),
            secret_key: "ddc2a396e5694fa7bb34b48ccc078abd".to_string(),
            passphrase: "".to_string(),
            url: "https://api.mexc.com".to_string(),
        };
        c = c.new_client(cfg);
        let r = c.time();
        println!("{:?}", r)
    }
    #[test]
    fn sign_test() {
        let mut c: Client = Client {
            api_key: "".to_string(),
            secret_key: "".to_string(),
            passphrase: "".to_string(),
            url: "".to_string(),
            client: Default::default(),
        };
        let cfg = Config {
            api_key: "mx0aBYs33eIilxBWC5".to_string(),
            secret_key: "45d0b3c26f2644f19bfb98b07741b2f5".to_string(),
            passphrase: "".to_string(),
            url: "https://api.mexc.com".to_string(),
        };
        c = c.new_client(cfg);
    }
}
