#[macro_use]
extern crate clap;
use clap::App;

use std::sync::Arc;

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder};

extern crate protos;
use models::orders;
use protos::refinery_grpc::RefineryClient;

fn main() {
    // Parse the command line first. Let's keep it simple...
    let cli_yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    println!("{:?}", matches);

    // Now let's talk to the grpc backend

    // Configure our client
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:55555").as_str());
    let client = RefineryClient::new(ch);

    // Here's where we create our proto objects, after parsing cli input

    if let Some(_matches) = matches.subcommand_matches("summary") {
        let empty_payload = protos::empty::Empty::new();

        // Send the gRPC message
        let orders = client.get_all_records(&empty_payload).expect("RPC Failed!");

        println!("Order status: {:?}", orders);
    }

    if let Some(matches) = matches.subcommand_matches("order") {
        let quantity = matches
            .value_of("quantity")
            .unwrap()
            .parse::<i32>()
            .expect("Quantity should be a number");
        println!("# of barrels to ship to refinery: {:?}", quantity);

        // Convert product to enum
        let product = matches.value_of("product").unwrap();

        println!("To be refined into: {:?}", product);

        let order = orders::OrderForm::new(quantity, product.to_string());

        //// Send the gRPC message. Convert our custom type to the proto form
        let order_status = client.order(&order.into()).expect("RPC Failed!");

        println!("Order status: {:?}", order_status);
    }
}
