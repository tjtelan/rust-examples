extern crate grpcio;
extern crate protos;

#[macro_use]
extern crate clap;
use clap::App;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use protobuf::RepeatedField;

use protos::refinery_grpc::RefineryClient;
use protos::refinery::{OrderForm, OrderStatus, OilProductType, OrderResponseType};
//
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


    //if let Some(matches) = matches.subcommand_matches("status") {
    //    let id = matches.value_of("orderid").unwrap().parse::<i32>().expect("ID should be a number");
    //    println!("Get logs for shipment number: {}", id);

    //    let mut orderid = OrderID::new();
    //    orderid.set_id(id);

    //    // Send the gRPC message
    //    let order_status = client.status(&orderid).expect("RPC Failed!");

    //    println!("Order status: {:?}", order_status);
    //}

    if let Some(matches) = matches.subcommand_matches("order") {

        let quantity = matches.value_of("quantity").unwrap().parse::<i32>().expect("Quantity should be a number");
        println!("# of barrels to ship to refinery: {:?}", quantity);

        // Convert product to enum
        let product = match matches.value_of("product").unwrap() {
            "gasoline" => OilProductType::GASOLINE,
            "jetfuel" => OilProductType::JETFUEL,
            "diesel" => OilProductType::DIESEL,
            "asphalt" => OilProductType::ASPHALT,
            "heavy" => OilProductType::HEAVY,
            "lubricant" => OilProductType::LUBRICANT,
            _ => OilProductType::OTHER,
        };
         
        println!("To be refined into: {:?}", product);


        // TODO: 

        // We're going to make an order
        // Build our data payload.
        let mut order = OrderForm::new();
        order.set_quantity(quantity);
        order.set_product(product);

        // Send the gRPC message
        let order_status = client.order(&order).expect("RPC Failed!");

        println!("Order status: {:?}", order_status);

    }
    
}
