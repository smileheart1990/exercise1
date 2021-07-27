extern crate lazy_static;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

//static  channel: (Sender<String>, Receiver<String>) = mpsc::channel();
//static  v: Vec<i32> = Vec::new();
pub struct Observer {
    actions: Vec<action>
}

// this is for single instance
static observer : Observer = Observer{
    actions:vec![store_msg{},print_price{}]
};

impl Observer{
    fn nonify_with_msg(msg: short_tweet_message){
        twitter_listener_service::
        for action in observer.actions{
            action.execute(msg);
        }
    }
}

trait action{
    fn execute(&self,msg:short_tweet_message){

    }
}

struct store_msg{

}

struct print_price{

}

impl action for store_msg{
    fn execute(&self, msg:short_tweet_message){
        println!("msg saved")
    }
}

impl action for print_price{
    fn execute(&self, msg:short_tweet_message){
        println!("print the price")
    }
}

