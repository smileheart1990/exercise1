use futures::prelude::*;
use twitter_stream::{Token, TwitterStream};


pub struct twitter_listner {}

impl twitter_listner {
    pub async fn start_twitter_listner_services() {
        // need to replace the key or token, or config that in the file.
        println!("hi");
        let token = Token::from_parts("yex95KUGuZi1Et7AwQNMIpooW", "DOtMcrVCSbUyVDFB7nKepUKWR2hrT1XGUFVkJNKegU25Cb71JX", "1379464579705618521-w9rVTAsbdTJLG5ENnIGvAPog72wRPQ", "AxBjSzE1wPIVWcGqS6BASJ0qdWPTjzOJcswK4QP5R1Lg5");
        TwitterStream::track("@Twitter", &token)
            .try_flatten_stream()
            .try_for_each(|json| {
                println!("{}", json);
                future::ok(())
            })
            .await
            .unwrap();
        println!("the twitter listen services up")
    }
}