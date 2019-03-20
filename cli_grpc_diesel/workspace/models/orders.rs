use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::pg::Pg;

#[derive(Queryable, Identifiable)]
pub struct Order {
    pub id: i32,
    pub quantity: i32,
    pub product_type : OilProductEnum,
}

#[derive(Insertable, Debug, PartialEq)]
#[table_name="orders"]
pub struct NewOrder {
    pub quantity: i32,
    pub product_type : OilProductEnum,
//    pub received_time : NaiveDateTime,
}
