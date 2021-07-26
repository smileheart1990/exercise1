
mod test2;
mod huobi_helper;
mod twitter_listener_service;
use std::thread::{sleep};
use std::time::Duration;
fn main(){
    /*let price = huobi_helper::huobi_help::get_latest_trade_price();
    match price {
        Ok(tmp)=>{
            match tmp{
                Some(tmp2)=>{println!("price is {}", tmp2)},
                _=>{println!("None var")}
            }
        },
        Err(error)=>{println!("error is {}", error)}
    }*/

    let successful_or_not = twitter_listener_service::twitter_listner::start_twitter_listner_services();

    sleep(Duration::from_millis(100000));
}