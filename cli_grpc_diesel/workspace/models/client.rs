use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::schema::{orders, OilProductEnum};
use crate::orders::{Order};
use chrono::{DateTime, Utc, NaiveDateTime};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_order(conn : &PgConnection, quantity : i32, product_type : OilProductEnum) -> Order {

    //let timestamp = NaiveDateTime::from_timestamp(0,Utc::now().timestamp_subsec_nanos());

    let new_order = vec![
		Order {
			id : 1,
        	quantity : quantity,
        	product_type : product_type,
		},
	];

    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(conn)
        .expect("Error saving new order")
}
