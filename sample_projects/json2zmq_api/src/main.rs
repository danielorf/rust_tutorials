#[macro_use] extern crate nickel;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use nickel::status::StatusCode;
use nickel::{Nickel, JsonBody, HttpRouter};

#[derive(Serialize, Deserialize)]
struct Person {
    first_name: String,
    last_name:  String,
    age:        u8,
}

fn main() {
    let mut server = Nickel::new();

    // try it with curl
    // curl 'http://localhost:6767' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "first_name": "John","last_name": "Connor", "age": 35 }'
    server.post("/", middleware! { |request, response|
        println!("Running niclkel and zmq...");

        // zmq stuff
        let context = zmq::Context::new();
        let requester = context.socket(zmq::REQ).unwrap();
        assert!(requester.connect("tcp://localhost:5555").is_ok());

        let person = try_with!(response, {
            request.json_as::<Person>().map_err(|e| (StatusCode::BadRequest, e))
        });
        println!("Sending message from json post to zmq: {} {} {}", person.first_name, person.last_name, person.age,);
        let age_x2 = person.age * 2;

        // Send the doubled age to the zmq server
        requester.send(&age_x2.to_string(), 0).unwrap();

        format!("Hello {} {} {}", person.first_name, person.last_name, person.age)
    });

    server.listen("127.0.0.1:6767").unwrap();
}
