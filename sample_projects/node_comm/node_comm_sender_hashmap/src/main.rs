use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    println!("Experiment with zmq event handler\n");

    // let mut vec = vec!["11111", "22222", "333", "44", "jgh", "kjgjhsdfj", "98789", "jkhsdf"];
    // let mut nodes: HashMap<&str, &str> = HashMap::new();
    // nodes.insert("id1", "hostname1");
    // nodes.insert("id2", "hostname2");
    // nodes.insert("id3", "hostname3");
    // nodes.insert("id4", "hostname4");
    let mut nodes: HashMap<String, String> = HashMap::new();
    nodes.insert("id1".to_string(), "hostname1".to_string());
    nodes.insert("id2".to_string(), "hostname2".to_string());
    nodes.insert("id3".to_string(), "hostname3".to_string());
    nodes.insert("id4".to_string(), "hostname4".to_string());

    // let serialized = serde_cbor::to_vec(&nodes).unwrap();
    let serialized = serde_json::to_string(&nodes).unwrap();
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    assert!(requester.connect("tcp://localhost:5555").is_ok());
    requester.send(&serialized, 0).unwrap();

    let mut msg = zmq::Message::new();
    requester.recv(&mut msg, 0).unwrap();
    
    // for node in &vec {
    //     requester.send(&node, 0).unwrap();
    //     requester.recv(&mut msg, 0).unwrap();
    //     thread::sleep(Duration::from_millis(1000));
    // }
}
