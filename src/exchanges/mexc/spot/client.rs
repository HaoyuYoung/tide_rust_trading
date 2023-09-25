use crate::exchanges::mexc::model::{QueryWithSignature, ServerTimeMEXC};
use crate::exchanges::model::{nil, Config, ServerTime};
use crate::exchanges::Clients;
use hmac::{Hmac, Mac};
use http::{HeaderMap, HeaderValue};
use reqwest::blocking::Response;
use sha2::Sha256;

pub struct Client {
    api_key: String,
    secret_key: String,
    passphrase: String,
    pub(crate) url: String,
    client: reqwest::blocking::Client,
}

impl Client {
    pub(crate) fn get<T>(
        &self,
        url: String,
        query: T,
    ) -> Result<Response, Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        let mut headers = HeaderMap::new();

        headers.insert("X-MEXC-APIKEY", HeaderValue::from_str(&self.api_key)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        let signed = self.sign_query(query)?;
        let res = self
            .client
            .get(url.as_str())
            .query(&signed)
            //.headers(headers)
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

impl Clients for Client {
    fn new_client(&self, c: Config) -> Self {
        Client {
            api_key: c.api_key,
            secret_key: c.secret_key,
            passphrase: c.passphrase,
            url: c.url,
            client: reqwest::blocking::Client::new(),
        }
    }
    fn ping(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/ping"));
        let res = self.get(url, nil {})?;
        Ok(res.status().is_success())
    }
    fn time(&self) -> Result<ServerTime, Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", self.url, String::from("/api/v3/time"));
        let res = self.get(url, nil {})?;
        let r: ServerTimeMEXC = res.json()?;
        Ok(ServerTime {
            server_time: r.server_time,
        })
    }
}
pub fn new_client_cfg(cfg: Config) -> Client {
    let c: Client = Client {
        api_key: "".to_string(),
        secret_key: "".to_string(),
        passphrase: "".to_string(),
        url: "".to_string(),
        client: Default::default(),
    };
    c.new_client(cfg)
}
#[cfg(test)]
mod tests {
    use crate::exchanges::mexc::spot::client::{Client, Clients};
    use crate::exchanges::model::Config;

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
            api_key: "".to_string(),
            secret_key: "".to_string(),
            passphrase: "".to_string(),
            url: "https://api.mexc.com".to_string(),
        };
        c = c.new_client(cfg);
        let r = c.time();
        println!("{:?}", r)
    }
}
