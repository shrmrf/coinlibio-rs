# coinlib.io-rs
[![Build Status](https://travis-ci.com/shrmrf/coinlibio-rs.svg?branch=master)](https://travis-ci.com/shrmrf/coinlibio-rs)

Rust implementation around coinlib.io API - just for fun and learning

### Running the integration test
Export your coinlib.io token and run test locally:

```bash
export COINLIB_TOKEN=<token-for-coinlib-api>
cargo test -- --ignored --nocapture
```

The `--nocapture` argument also prints the output of the API call.
