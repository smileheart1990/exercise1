/*
mod test2;
mod huobi_helper;
mod twitter_listener_service;
use std::thread::{sleep};
use std::time::Duration;
use futures::executor::block_on;
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

    block_on(twitter_listener_service::twitter_listner::start_twitter_listner_services());



    sleep(Duration::from_millis(100000));
}*/
use std::thread;
use std::time::Duration;

use futures::prelude::*;
use twitter_stream::{Token, TwitterStream};
mod twitter_listener_service;
use futures::executor::block_on;
#[tokio::main]
async fn main() {
    let twitter_listener_service = twitter_listener_service::twitter_listner::start_twitter_listner_services();

    println!("twitter listener services up");
   /* let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });*/

    block_on(twitter_listener_service);

}