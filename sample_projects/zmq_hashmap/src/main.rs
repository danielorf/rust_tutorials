// #![crate_name = "try_zmq_client"]
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use serde_cbor;
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    hue: String,
    val: i8,
}

fn main() {
    println!("Experiment to handle hashmaps with zmq\n");

    // let mut timber_resources: HashMap<&str, i32> = HashMap::new();
    // timber_resources.insert("Norway", 199);
    // timber_resources.insert("Denmark", 50);
    // timber_resources.insert("Iceland", 10);
    // // println!("Norway's timer resources: {}",timber_resources.get(&"Norway").unwrap());

    // let serialized = serde_json::to_string(&timber_resources).unwrap();
    // // let serialized = serde_json::to_vec(&timber_resources).unwrap();
    // // println!("serialized = {}", serialized);

    let mut point_thing: HashMap<&str, Point> = HashMap::new();
    point_thing.insert("Norway", Point { x: 4336, y: 2463, color: Color { hue: "greenish".to_string(), val: 102 } });
    point_thing.insert("Iceland", Point { x: 765, y: 0067, color: Color { hue: "bluekinda".to_string(), val: 45 } });
    // println!("norwayy: {:?}", point_thing.get(&"Norway").unwrap());
    // let serialized = serde_json::to_vec(&point_thing).unwrap();
    let serialized = serde_cbor::to_vec(&point_thing).unwrap();


    // Server
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    assert!(responder.bind("tcp://*:5555").is_ok());

    // Client
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();


    requester.send(&serialized, 0).unwrap();

    responder.recv(&mut msg, 0).unwrap();
    
    // let deserialized: HashMap<&str, i32> = serde_json::from_str(&serialized).unwrap();
    // let deserialized: HashMap<&str, i32> = serde_json::from_slice(&serialized).unwrap();
    // let deserialized: HashMap<&str, Point> = serde_json::from_str(&serialized).unwrap();
    // let deserialized: HashMap<&str, Point> = serde_json::from_slice(&serialized).unwrap();
    let deserialized: HashMap<&str, Point> = serde_cbor::from_slice(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized.get(&"Norway").unwrap());
    println!("deserialized = {:?}", deserialized);
    println!("deserialized = {:?}", deserialized.get(&"Iceland").unwrap().color.hue);


    // println!("Received {}", msg.as_str().unwrap());

}
