extern crate serde_json;
#[macro_use]
extern crate serde_derive;
// use serde_cbor;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    id: String,                                 // Unique ID
    host: String,                               // IP address or FQDN
    known_nodes: HashMap<String, String>,       // Format is (id, host)
    adjacent: HashMap<String, DateTime<Utc>>,   // Contains vector of ids to minimize storage
    delinquent: HashMap<String, DateTime<Utc>>, // Format is (id, time_reported)
}

impl Node {
    fn new(id: &str, host: &str) -> Node {
        Node {
            id: id.to_string(),
            host: host.to_string(),
            known_nodes: HashMap::new(), //HashMap::<String, String>::new(),
            adjacent: HashMap::new(),    //HashMap<&str, DateTime<UTC>>,
            delinquent: HashMap::new(),  //HashMap<&str, DateTime>,
        }
    }

    fn run(&self) {
        let allowed_duration = Duration::new(2, 0);
        // let mut node_vec: Vec<String> = Vec::new();

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
                let deserialized: HashMap<&str, &str> =
                    serde_json::from_str(&message.as_str().unwrap()).unwrap();
                println!("Received {:?}", deserialized);
                responder.send("", 0).unwrap();
            }
            // if start_time.elapsed() > allowed_duration {
            //     // if node_vec.len() > 0 {
            //     //     println!("{:?}", node_vec);
            //     // }
            //     // println!("{:?}", node_vec);
            //     start_time = Instant::now();
            // }
        }
    }

    // TO BE IMPLEMENTED
    // fn get_adj_sample()
    // fn join()
    // fn add_node()
    // fn send_message()
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Join = 0,
    Remove,
    Gossip,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    target: String,
    sender: String,
    msg_type: MessageType,
    payload: Vec<String>,
}

fn main() {
    println!("Initial representation of a running Node");

    let myself = Node::new("myid", "myhostname");
    myself.run();
    // println!("{}", myself.id)
}
