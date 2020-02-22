use crate::schema::{orders, OilProductEnum};
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct Order {
    pub id: i32,
    pub quantity: i32,
    pub product_type: OilProductEnum,
    pub received_time: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[table_name = "orders"]
pub struct NewOrder {
    pub quantity: i32,
    pub product_type: OilProductEnum,
    pub received_time: NaiveDateTime,
}

// This struct is used on the client-side
pub struct OrderForm {
    pub quantity: i32,
    pub product_type: OilProductEnum,
}

// This struct is used on the client-side
//pub struct OrderQuery {
//    pub id : Option<i32>,
//    pub product_type : Option<OilProductEnum>,
//    pub before_time: Option<NaiveDateTime>,
//    pub after_time: Option<NaiveDateTime>,
//}
