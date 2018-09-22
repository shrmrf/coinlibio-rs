extern crate coinlibio;

use coinlibio::coinlib::{CoinlibApi, CoinlibAuth, Endpoint, EndpointParams};

#[test]
#[ignore]
fn test_coinlib() {
    match std::env::vars().find(|(name, value)| name == &"COINLIB_TOKEN".to_string()) {
        None => panic!("export COINLIB_TOKEN=<token>"),
        Some((_name, value)) => {
            let auth = CoinlibAuth::new(&value);
            let api = CoinlibApi::new(&auth.unwrap()).unwrap();

            println!(
                "{}",
                api.request(
                    Endpoint::Coin,
                    EndpointParams {
                        currency: "USD".to_string(),
                        symbol: "BTC".to_string()
                    }
                ).unwrap()
            );
        }
    }
}
