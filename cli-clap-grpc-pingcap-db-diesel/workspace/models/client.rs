use chrono::{NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::orders::{NewOrder, Order, OrderForm};
use crate::schema::{orders, OilProductEnum};

use protos::refinery;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// create_order is used by the backend. This is one of the test cases for converting the proto type
// into custom type with From/Into traits whenever we need to use it.
pub fn create_order(conn: &PgConnection, order_form: OrderForm) -> Order {
    let new_order = vec![NewOrder {
        quantity: order_form.quantity,
        product_type: order_form.product_type,
        received_time: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
    }];

    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(conn)
        .expect("Error saving new order")
}

// get_all_orders is used by the backend
pub fn get_all_orders(conn: &PgConnection) -> Vec<Order> {
    let query: Vec<Order> = orders::table
        .select(orders::all_columns)
        .order_by(orders::id)
        .load(conn)
        .expect("Error getting all order records");

    query
}

// order_received_success is used by the backend. The proto typed result is returned to the caller
pub fn order_received_success() -> refinery::OrderStatus {
    let mut order_status = refinery::OrderStatus::new();
    order_status.set_status(refinery::OrderResponseType::RECEIVED);
    order_status
}

// db_query_to_proto is used by the backend to convert a Vector of Order (from a Diesel select
// query) into the proto native OrderRecordList. Implementing `From` for a Vector would have taken
// longer, and used a wrapper type. That very well may be the more maintainable approach, but this
// was quicker...
pub fn db_query_to_proto(rust_record: Vec<Order>) -> refinery::OrderRecordList {
    let mut proto_vec: Vec<refinery::OrderRecord> = Vec::new();

    // Let's take advantage of the `From` trait
    for r in rust_record {
        proto_vec.push(refinery::OrderRecord::from(r));
    }

    let proto_order = protobuf::RepeatedField::from_vec(proto_vec);

    let mut proto_final = refinery::OrderRecordList::new();
    proto_final.set_order(proto_order);
    proto_final
}

impl OrderForm {
    // new is used on the client-side. This is for taking in command line input from a client
    pub fn new(quantity: i32, product_type: String) -> Self {
        println!("# of barrels to ship to refinery: {:?}", quantity);

        // Convert product to enum
        // <String>.as_ref() used bc the left-hand side is &str
        let product_type_parsed = match product_type.as_ref() {
            "gasoline" => OilProductEnum::GASOLINE,
            "jetfuel" => OilProductEnum::JETFUEL,
            "diesel" => OilProductEnum::DIESEL,
            "asphalt" => OilProductEnum::ASPHALT,
            "heavy" => OilProductEnum::HEAVY,
            "lubricant" => OilProductEnum::LUBRICANT,
            _ => OilProductEnum::OTHER,
        };

        println!("To be refined into: {:?}", product_type_parsed);

        OrderForm {
            quantity: quantity,
            product_type: product_type_parsed,
        }
    }
}
