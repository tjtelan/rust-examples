#[macro_use]
extern crate clap;
use clap::App;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

extern crate models;
use models::{client, orders};

extern crate protos;
use protos::refinery;
use protos::refinery_grpc::{self, Refinery}; // `self` is *probably* for ::create_refinery

// This is the start of our local implementation of the gRPC service using the protobuf spec
#[derive(Clone)]
struct RefineryService;

//// We're going to implement the Refinery trait on our own struct.
//// These are the methods used by the client.
impl Refinery for RefineryService {
    fn order(&mut self, ctx: RpcContext, req: refinery::OrderForm, sink: UnarySink<refinery::OrderStatus>) {

        println!("Received an order: {:?}", req);

        // Creating the return object
        let order_status = client::order_received_success();

        // ???: Map successful result on the Sink
        let f = sink
            .success(order_status.clone())
            .map(move |_| println!("Responded with status {{ {:?} }}", order_status))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));

        // FIXME: Instead, can I connect to db before registering this service? Is that thread-safe?
        let conn = client::establish_connection();
        // Convert the received proto request into our native type
        let _new_order = client::create_order(&conn, orders::OrderForm::from(req));

        ctx.spawn(f)
    }
}

//    fn status(&mut self, ctx: RpcContext, req: OrderID, sink: UnarySink<OrderRecord>) {
//        println!("Request for status on order ID: {:?}", req);
//
//        // Here's where a DB query would happen to return any existing records.
//
//        // Building the record manually for now
//        let mut order_record = OrderRecord::new();
//
//        // We could probably just choose random values as demonstration
//        let mut order_form = OrderForm::new();
//        order_form.set_quantity(99);
//        order_form.set_product(OilProductType::LUBRICANT);
//
//        // Attach to the record we are returning
//        order_record.set_id(req);
//        order_record.set_order(order_form);
//
//        // ???: Map successful result on the Sink
//        let f = sink
//            .success(order_record.clone())
//            .map(move |_| println!("Status: {{ {:?} }}", order_record))
//            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
//
//        ctx.spawn(f)
//    }
//}

fn main() {

    // Parse the command line first. Let's keep it simple...
    let cli_yaml = load_yaml!("backend.yaml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    println!("{:?}", matches);

    // Now let's set up the gRPC server with our local implementation of the Refinery trait
    // ServerBuilder takes in a grpcio::Environment (for threadpooling),
    // and registers our service.
    // Our service, which implements the Refinery trait created by the protobuf service declaration
    let env = Arc::new(Environment::new(1));
    let service = refinery_grpc::create_refinery(RefineryService);

    // FIXME: The bind addr and port should be configurable
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 55555)
        .build()
        .unwrap();
    server.start();

    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    // This is just user input stuff for interactive testing. 
    // TODO: How does this work? 
    // We create a channel
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });

    let _ = rx.wait();
    // We'll only get here if we get the specified user action.
    println!("I'm about to die!");
    let _ = server.shutdown().wait();

}

