extern crate reqwest;

use reqwest::blocking::Response;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

pub struct huobi_help {}

impl huobi_help {
    //bellow is huobi reqeust demo
    //curl "https://api.huobi.pro/market/trade?symbol=ethusdt"
/*    {
    "ch": "market.ethusdt.trade.detail",
    "status": "ok",
    "ts": 1627197506785,
    "tick": {
    "id": 132094344820,
    "ts": 1627197506781,
    "data": [
    {
    "id": 132094344820326748853638281,
    "ts": 1627197506781,
    "trade-id": 102261656795,
    "amount": 0.01,
    "price": 2178.28,
    "direction": "sell"
    },
    {
    "id": 132094344820326748904918949,
    "ts": 1627197506781,
    "trade-id": 102261656794,
    "amount": 0.0034,
    "price": 2178.33,
    "direction": "sell"
    }
    ]
    }
    }*/
    fn get_lastest_trade_response() -> std::result::Result<String, Box<dyn std::error::Error>> {
        let body = reqwest::blocking::get("https://api.huobi.pro/market/trade?symbol=ethusdt")?
            .text()?;
        println!("recived from latest api from huobi = {:?}", body);
        Ok(body)
    }

    pub fn get_latest_trade_price() -> std::result::Result<Option<f64>, Box<dyn std::error::Error>> {
        let json_str = huobi_help::get_lastest_trade_response()?;
        //let json_str = String::from(r#"{"ch":"market.ethusdt.trade.detail","status":"ok","ts":1627194345499,"tick":{"id":132092321222,"ts":1627194345285,"data":[{"id":132092321222326748451900135,"ts":1627194345285,"trade-id":102261637625,"amount":0.1312,"price":2164.85,"direction":"sell"}]}}"#);
        let latest_response: Value = serde_json::from_str(&json_str)?;
        println!("current price {} ", latest_response["tick"]["data"][0]["price"]);
        Ok(latest_response["tick"]["data"][0]["price"].as_f64())
    }
}