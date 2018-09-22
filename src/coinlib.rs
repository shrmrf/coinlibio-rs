extern crate reqwest;
extern crate serde_json;
use std;
use std::error::Error;

macro_rules! coinlib_url {
    ($key:expr) => {{
        format!(
            "https://coinlib.io/api/v1/coin?key={api_key}&pref={curr}&symbol={coin}",
            api_key = $key,
            curr = "USD",
            coin = "BTC"
        )
    }};
    ($key:expr, $coin:expr) => {{
        format!(
            "https://coinlib.io/api/v1/coin?key={api_key}&pref={curr}&symbol={coin}",
            api_key = $key,
            curr = "USD",
            coin = $coin
        )
    }};
    ($key:expr, $coin:expr, $currency:expr) => {{
        format!(
            "https://coinlib.io/api/v1/coin?key={api_key}&pref={curr}&symbol={coin}",
            api_key = $key,
            curr = $currency,
            coin = $coin
        )
    }};
}

pub enum Endpoint {
    Global,
    Coinlist,
    Coin,
}

pub struct EndpointParams {
    pub currency: String,
    pub symbol: String,
}

#[test]
fn auth_init_test() {
    let auth = CoinlibAuth::new("abcdef");
    assert_eq!(auth.unwrap().api_key, "abcdef");
}

pub struct CoinlibAuth {
    api_key: String,
}

impl CoinlibAuth {
    pub fn new(key: &str) -> Result<(CoinlibAuth), Box<Error>> {
        let coinlib_auth = CoinlibAuth {
            api_key: key.to_string(),
        };

        Ok(coinlib_auth)
    }
}

pub struct CoinlibApi {
    api_key: String,
}

impl CoinlibApi {
    pub fn new(creds: &CoinlibAuth) -> Result<(CoinlibApi), Box<Error>> {
        Ok(CoinlibApi {
            api_key: creds.api_key.to_string(),
        })
    }

    pub fn request(
        &self,
        endpoint: Endpoint,
        params: EndpointParams,
    ) -> Result<(String), Box<Error>> {
        let req;

        match endpoint {
            Endpoint::Coin => req = coinlib_url!(self.api_key, params.symbol, params.currency),

            // Todo: implement other endpoints
            _ => panic!("unimplemented"),
        }

        let client;
        match std::env::vars().find(|(name, value)| name == &"http_proxy".to_string()) {
            None => client = reqwest::Client::new(),

            Some((_name, value)) => {
                println!("{}", value);
                let url = reqwest::Proxy::all(&value)?;
                client = reqwest::Client::builder().proxy(url).build()?
            }
        }

        let mut response = client.get(&req).send()?;
        let coin_info = response.text()?;

        Ok(coin_info.to_string())
    }
}

#[derive(Debug)]
pub struct Coinlib;

impl Coinlib {
    pub fn new(coinlib_creds: &CoinlibAuth) -> Result<(Box<CoinlibApi>), Box<Error>> {
        Ok(Box::new(CoinlibApi::new(coinlib_creds)?))
    }
}
