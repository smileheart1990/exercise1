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

use futures::prelude::*;
use twitter_stream::{Token, TwitterStream};

#[tokio::main]
async fn main() {
    println!("hi");
    let token = Token::from_parts("yex95KUGuZi1Et7AwQNMIpooW", "DOtMcrVCSbUyVDFB7nKepUKWR2hrT1XGUFVkJNKegU25Cb71JX", "1379464579705618521-w9rVTAsbdTJLG5ENnIGvAPog72wRPQ", "AxBjSzE1wPIVWcGqS6BASJ0qdWPTjzOJcswK4QP5R1Lg5");
    TwitterStream::follow(&[1418481330120056833], &token)
        .try_flatten_stream()
        .try_for_each(|json| {
            println!("{}", json);
            future::ok(())
        })
        .await
        .unwrap();
    /*TwitterStream::track("@Twitter", &token)
        .try_flatten_stream()
        .try_for_each(|json| {
            println!("{}", json);
            future::ok(())
        })
        .await
        .unwrap();*/
}