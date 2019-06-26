use std::thread;
use std::time::Duration;

fn main() {
    println!("Experiment with zmq event handler\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    let mut vec = vec!["11111", "22222", "333", "44", "jgh", "kjgjhsdfj", "98789", "jkhsdf"];

    // for request_nbr in 0..10 {
    //     println!("Sending Hello {}...", request_nbr);
    //     requester.send("Hellow", 0).unwrap();
    //     requester.recv(&mut msg, 0).unwrap();
    //     // requester
    // }
    // requester.send("erweewrwe", 0).unwrap();
    
    // let mut counter: u8 = 1;
    // requester.send(&counter.to_string(), 0).unwrap();
    // loop {
    //     requester.send(&counter.to_string(), 0).unwrap();
    //     requester.recv(&mut msg, 0).unwrap();
    //     counter = counter + 1;
    //     thread::sleep(Duration::from_millis(2000));
    // }

    for node in &vec {
        requester.send(&node, 0).unwrap();
        requester.recv(&mut msg, 0).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}
