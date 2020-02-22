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
use models::client;

extern crate protos;
use protos::refinery;
use protos::refinery_grpc::{self, Refinery}; // `self` is *probably* for ::create_refinery

// This is the start of our local implementation of the gRPC service using the protobuf spec
#[derive(Clone)]
struct RefineryService;

impl Refinery for RefineryService {
    // The client-side converts to refinery::OrderForm while calling this endpoint.
    // But we convert the proto type back to our custom type right before adding to the database
    fn order(
        &mut self,
        ctx: RpcContext,
        req: refinery::OrderForm,
        sink: UnarySink<refinery::OrderStatus>,
    ) {
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
        let _new_order = client::create_order(&conn, req.into());

        ctx.spawn(f)
    }

    fn get_all_records(
        &mut self,
        ctx: RpcContext,
        _req: protos::empty::Empty,
        sink: UnarySink<refinery::OrderRecordList>,
    ) {
        println!("Received request for all of the order records");

        // FIXME: Instead, can I connect to db before registering this service? Is that thread-safe?
        let conn = client::establish_connection();

        // Call out to db
        let query_results = client::get_all_orders(&conn);

        // FIXME: This conversion pattern is different than the plain `From` traits, because we
        // have to handle the outer vector in a special way.
        let parsed_query_proto = client::db_query_to_proto(query_results);
        //println!("Got results from the database: {:?}", query_results);

        let f = sink
            .success(parsed_query_proto.clone())
            .map(move |_| {
                println!(
                    "Responded with list of records {{ {:?} }}",
                    parsed_query_proto
                )
            })
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));

        ctx.spawn(f)
    }
}

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
