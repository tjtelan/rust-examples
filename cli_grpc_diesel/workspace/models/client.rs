use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::{Utc, NaiveDateTime};

use crate::schema::{orders,OilProductEnum};
use crate::orders::{Order,NewOrder,OrderForm};

use protos::refinery;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_order(conn : &PgConnection, order_form : OrderForm) -> Order {
    let timestamp = NaiveDateTime::from_timestamp(Utc::now().timestamp(),0);

    let new_order = vec![
        NewOrder {
            quantity : order_form.quantity,
            product_type : order_form.product_type,
            received_time : timestamp,
        },
    ];

    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(conn)
        .expect("Error saving new order")
}

pub fn order_received_success() -> refinery::OrderStatus {
    let mut order_status = refinery::OrderStatus::new();
    order_status.set_status(refinery::OrderResponseType::RECEIVED);
    order_status
}

impl OrderForm {
    // new is used on the client-side. This is for taking in command line input from a client
    pub fn new(quantity : i32, product_type : String) -> Self {

        println!("# of barrels to ship to refinery: {:?}", quantity);

        // Convert product to enum
        let product_type_parsed = match product_type.as_ref() {
            "gasoline"  => OilProductEnum::GASOLINE,
            "jetfuel"   => OilProductEnum::JETFUEL,
            "diesel"    => OilProductEnum::DIESEL,
            "asphalt"   => OilProductEnum::ASPHALT,
            "heavy"     => OilProductEnum::HEAVY,
            "lubricant" => OilProductEnum::LUBRICANT,
            _           => OilProductEnum::OTHER,
        };
         
        println!("To be refined into: {:?}", product_type_parsed);

        OrderForm {
            quantity : quantity,
            product_type : product_type_parsed,
        }
    }
}
