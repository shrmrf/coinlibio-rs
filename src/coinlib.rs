extern crate reqwest;
extern crate serde_json;
use std;
use std::error::Error;

#[cfg(test)]
#[test]
fn it_works() {
    let coinlib_handler = Coinlib::new("69fd5168e0847c19", "BTC", "USD");
    let request_url = format!(
        "https://coinlib.io/api/v1/coin?key={key}&pref=USD&symbol={ticker}",
        key = "69fd5168e0847c19",
        ticker = "BTC"
    );

    let client;
    match std::env::vars().find(|(name, value)| name == &"http_proxy".to_string()) {
        None => client = reqwest::Client::new(),

        // Todo: improve this check
        Some((_name, value)) => {
            println!("{}", value);
            let url = reqwest::Proxy::all(&value).unwrap();
            println!("{:?}", value);
            // Todo: fix the &value --> url conversion
            client = reqwest::Client::builder().proxy(url).build().unwrap()
        }
    }
    let mut response = client.get(&request_url).send().unwrap(); //reqwest::get(&request_url)?;

    // if !response.status().is_success() {
    //     Err(format!("{}", response.status()));
    // }

    let coin_info = response.text().unwrap();
    //println!("{}", coin_info);

    use serde_json::{Error, Value};
    let v: Value = serde_json::from_str(&coin_info).unwrap();
    println!(
        r#"BTC:
            {}USD
            rank: {}
            %change (7d): {}%
        "#,
        v["price"], v["rank"], v["delta_7d"]
    );

    assert_eq!(2 + 2, 4);
}

#[derive(Debug)]
pub struct Coinlib {
    api_key: String,
    ticker_sym: String,
    pref_curr: String,
}

impl Coinlib {
    pub fn new(api_key: &str, ticker: &str, currency: &str) -> Result<(Coinlib), Box<Error>> {
        let coinlib_api = Coinlib {
            api_key: api_key.to_string(),
            ticker_sym: ticker.to_string(),
            pref_curr: currency.to_string(),
        };

        Ok(coinlib_api)
    }
}
