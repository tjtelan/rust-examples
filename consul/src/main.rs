extern crate consul;

// This crate might need to be updated to support updates to Consul's endpoints
use consul::{Client, Keystore, Service};
use std::collections::HashMap;

fn main() {
    dotenv::dotenv().ok();

    let client = Client::new("http://127.0.0.1:8500");

    // Print services
    // This doesn't seem to return the default 'consul' service
    let services: HashMap<String, Service> = client.agent.services().unwrap();
    println!("{:?}", services);

    // Print members
    let members = client.agent.members();

    // Thie crate needs to be updated to derive Debug for this struct
    //println!("{:?}", members);

    // Write a value to the keystore
    let keystore = Keystore::new("http://127.0.0.1:8500");
    let key: String = "foo".to_string();
    let value: String = "var".to_string();

    let result = keystore.set_key(key.clone(), value);
    assert!(result.is_ok());

    let result = keystore.get_key(key).unwrap().unwrap();
	assert_eq!(result, "dmFy"); // Base64 encoded
}
