use futures::prelude::*;
use twitter_stream::{Token, TwitterStream};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::thread;
mod observer;
// this is just a short version, the tweet api return more fields, refer to tweet.json for more detail
struct short_tweet_message {
    id: String,
    text: String,
    time: String,
}

pub struct twitter_listner {}

impl twitter_listner {
    pub async fn start_twitter_listner_services() {
        // 44196397 is musk elonmusk id
        // 1418481330120056833 is my id, and this is just for test
        // we can get the id from https://tweeterid.com/, thanks for this web.
        let token = Token::from_parts("yex95KUGuZi1Et7AwQNMIpooW", "DOtMcrVCSbUyVDFB7nKepUKWR2hrT1XGUFVkJNKegU25Cb71JX", "1379464579705618521-w9rVTAsbdTJLG5ENnIGvAPog72wRPQ", "AxBjSzE1wPIVWcGqS6BASJ0qdWPTjzOJcswK4QP5R1Lg5");
        TwitterStream::follow(&[1418481330120056833], &token)
            .try_flatten_stream()
            .try_for_each(|json| {
                println!("recieved {}", json);
                let handle = thread::spawn(|| {
                    let short_tweet_message_for_serde: Value = serde_json::from_str(&json)?;
                    let short_tweet_message = short_tweet_message {
                        id: String::from(short_tweet_message_for_serde["id_str"].as_str()),
                        text: String::from(short_tweet_message_for_serde["text"].as_str()),
                        time: String::from(short_tweet_message_for_serde["created_at"].as_str()),
                    };
                    Observer::
                });


                future::ok(())
            }).await.unwrap();
    }
}