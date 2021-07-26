extern crate reqwest;
use  reqwest::blocking::Response;
use reqwest::Error;
use json::object;
pub struct huobi_help{
}
pub struct lastest_trade_response{
    id: String,
    trade_id: u64,
    price : f64,
    amout: u8,
    direction: String

}
impl huobi_help {
    //bellow is huobi reqeust demo
    //curl "https://api.huobi.pro/market/trade?symbol=ethusdt"
    //{
    //     "id": 600848670,
    //     "ts": 1489464451000,
    //     "data": [
    //       {
    //         "id": 600848670,
    //         "trade-id": 102043494568,
    //         "price": 7962.62,
    //         "amount": 0.0122,
    //         "direction": "buy",
    //         "ts": 1489464451000
    //       }
    //     ]
    // }
    pub fn lastest_trade_response()-> Result<String,String>{
        let body = reqwest::blocking::get("https://api.huobi.pro/market/trade?symbol=ethusdt")?
            .text()?;
        println!("recived from latest api from huobi = {:?}", body);
        OK(body)
    }

    fn get_latest_trade_price()-> lastest_trade_response{
        let instantiated = object!{
        "id" => 200,
        "success" => true,
        "payload" => object!{
            "features" => array![
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };
    }
}