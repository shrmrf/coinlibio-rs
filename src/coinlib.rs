extern crate reqwest;
extern crate serde_json;
use std;
use std::error::Error;

#[test]
fn auth_init_test() {
    let auth = CoinlibAuth::new("abcdef");
    assert_eq!(auth.unwrap().api_key, "abcdef");
}

pub struct CoinlibAuth {
    api_key: String,
}

impl CoinlibAuth {
    fn new(key: &str) -> Result<(CoinlibAuth), Box<Error>> {
        let coinlib_auth = CoinlibAuth {
            api_key: key.to_string(),
        };

        Ok(coinlib_auth)
    }
}

#[test]
fn test_api_string_creation() {
    let auth = CoinlibAuth::new("69fd5168e0847c19");
    let api = CoinlibApi::new(&auth.unwrap()).unwrap();
    assert_eq!(
        api.get_call_str("BTC", "USD").unwrap(),
        "https://coinlib.io/api/v1/coin?key=69fd5168e0847c19&pref=USD&symbol=BTC"
    );
}

pub struct CoinlibApi {
    api_key: String,
}

impl CoinlibApi {
    fn new(creds: &CoinlibAuth) -> Result<(CoinlibApi), Box<Error>> {
        Ok(CoinlibApi {
            api_key: creds.api_key.to_string(),
        })
    }

    fn get_call_str(&self, ticker: &str, currency: &str) -> Result<(String), Box<Error>> {
        Ok(format!(
            "https://coinlib.io/api/v1/coin?key={key}&pref={curr}&symbol={ticker}",
            key = self.api_key.to_string(),
            ticker = ticker.to_string(),
            curr = currency.to_string()
        ))
    }
}

#[derive(Debug)]
pub struct Coinlib;

impl Coinlib {
    pub fn new(coinlib_creds: &CoinlibAuth) -> Result<(Box<CoinlibApi>), Box<Error>> {
        Ok(Box::new(CoinlibApi::new(coinlib_creds)?))
    }
}
