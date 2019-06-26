// #[macro_use]
// extern crate serde_json;
// #[macro_use]
// extern crate serde_derive;
// use serde_cbor;
// use std::collections::HashMap;
use std::thread;
// use std::time::Duration;

use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::{Duration, Instant};

// #[derive(Serialize, Deserialize, Debug)]
// struct Message {
//     target: String,
//     val: i8,
// }

fn main() {
    println!("Experiment with zmq event handler\n");
    let allowed_duration = Duration::new(2, 0);
    let mut msg = zmq::Message::new();
    let mut node_vec: Vec<String> = Vec::new();

    // Server setup
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut start_time = Instant::now();
    loop {
        if responder
            .poll(zmq::POLLIN, 10)
            .expect("client failed polling")
            > 0
        {
            let message = responder.recv_msg(0).unwrap();
            node_vec.push(message.as_str().unwrap().to_string());
            responder.send("", 0).unwrap();
        }
        if start_time.elapsed() > allowed_duration {
            if node_vec.len() > 0 {
                println!("{:?}", node_vec);
            }
            start_time = Instant::now();
        }
    }
}
