// use std::thread;
// use std::time::Duration;
use serde_cbor;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Join = 0,
    Remove,
    Gossip,
    Sync,
    Ping,
    Health,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    target: String,
    sender: String,
    msg_type: MessageType,
    payload: Vec<String>, // Maybe change to something more JSON friendly
}

fn main() {
    println!("Experiment with sending gary Message\n");

    let mut vec = vec![
        "11111".to_string(),
        "22222".to_string(),
        "333".to_string(),
        "44".to_string(),
        "jgh".to_string(),
        "kjgjhsdfj".to_string(),
        "98789".to_string(),
        "jkhsdf".to_string(),
    ];
    let msg = Message {
        target: "abcdefg".to_string(),
        sender: "id823798".to_string(), // ToDo:  Fix to use a ref instead
        msg_type: MessageType::Gossip,
        payload: vec.clone(), // ToDo:  Fix to use a ref instead - https://matklad.github.io/2018/05/04/encapsulating-lifetime-of-the-field.html
    };

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let serialized = serde_cbor::to_vec(&msg).unwrap();
    requester.send(&serialized, 0).unwrap();

    let ack = requester.recv_string(0).unwrap().unwrap();
    println!("Received {:?}", ack);

    // let ack = requester.recv_bytes(0).unwrap();
    // println!("Received {:?}", String::from_utf8(ack).unwrap());
}
