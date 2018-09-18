extern crate reqwest;
extern crate serde_json;
use std;
use std::error::Error;

// #[cfg(test)]
// #[test]
// fn it_works() {
//     let coinlib_handler = Coinlib::new("69fd5168e0847c19", "BTC", "USD");
//     let request_url = format!(
//         "https://coinlib.io/api/v1/coin?key={key}&pref=USD&symbol={ticker}",
//         key = "69fd5168e0847c19",
//         ticker = "BTC"
//     );

//     let client;
//     match std::env::vars().find(|(name, value)| name == &"http_proxy".to_string()) {
//         None => client = reqwest::Client::new(),

//         // Todo: improve this check
//         Some((_name, value)) => {
//             println!("{}", value);
//             let url = reqwest::Proxy::all(&value).unwrap();
//             println!("{:?}", value);
//             // Todo: fix the &value --> url conversion
//             client = reqwest::Client::builder().proxy(url).build().unwrap()
//         }
//     }
//     let mut response = client.get(&request_url).send().unwrap(); //reqwest::get(&request_url)?;

//     // if !response.status().is_success() {
//     //     Err(format!("{}", response.status()));
//     // }

//     let coin_info = response.text().unwrap();
//     //println!("{}", coin_info);

//     use serde_json::{Error, Value};
//     let v: Value = serde_json::from_str(&coin_info).unwrap();

//     println!(
//         r#"BTC:
//             {}USD
//             rank: {}
//             %change (7d): {}%
//         "#,
//         v["price"], v["rank"], v["delta_7d"]
//     );

//     assert_eq!(2 + 2, 4);
// }

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

pub struct CoinlibApi {
    call_string: String,
}

impl CoinlibApi {
    fn new(creds: &CoinlibAuth) -> Result<(CoinlibApi), Box<Error>> {
        Ok(CoinlibApi {
            call_string: format!(
                "https://coinlib.io/api/v1/coin?key={key}&pref=USD&symbol={ticker}",
                key = creds.api_key,
                ticker = "BTC"
            ),
        })
    }
}

#[derive(Debug)]
pub struct Coinlib;

impl Coinlib {
    pub fn new(coinlib_creds: &CoinlibAuth) -> Result<(Box<CoinlibApi>), Box<Error>> {
        Ok(Box::new(CoinlibApi::new(coinlib_creds)?))
    }
}
