use crate::schema::{orders,OilProductEnum};
use chrono::NaiveDateTime;

use protos::refinery;

#[derive(Queryable, Identifiable)]
pub struct Order {
    pub id: i32,
    pub quantity: i32,
    pub product_type : OilProductEnum,
    pub received_time : NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[table_name="orders"]
pub struct NewOrder {
    pub quantity: i32,
    pub product_type : OilProductEnum,
    pub received_time : NaiveDateTime,
}

// This struct is used on the client-side
pub struct OrderForm {
    pub quantity : i32,
    pub product_type : OilProductEnum,
}

// Convert from the protos to our type
impl From<refinery::OrderForm> for OrderForm {
    fn from(proto_form : refinery::OrderForm) -> Self {
        let product_parsed = match proto_form.get_product() {
            refinery::OilProductType::GASOLINE  => OilProductEnum::GASOLINE,
            refinery::OilProductType::JETFUEL   => OilProductEnum::JETFUEL,
            refinery::OilProductType::DIESEL    => OilProductEnum::DIESEL,
            refinery::OilProductType::ASPHALT   => OilProductEnum::ASPHALT,
            refinery::OilProductType::HEAVY     => OilProductEnum::HEAVY,
            refinery::OilProductType::LUBRICANT => OilProductEnum::LUBRICANT,
            refinery::OilProductType::OTHER     => OilProductEnum::OTHER,
        };
 
        OrderForm {
            quantity : proto_form.get_quantity(),
            product_type : product_parsed,
        }
    }
}

// Convert from our type to the proto
impl From<OrderForm> for refinery::OrderForm {
    fn from(rust_form : OrderForm) -> Self {
        let product_parsed = match rust_form.product_type {
            OilProductEnum::GASOLINE  => refinery::OilProductType::GASOLINE,
            OilProductEnum::JETFUEL   => refinery::OilProductType::JETFUEL,
            OilProductEnum::DIESEL    => refinery::OilProductType::DIESEL,
            OilProductEnum::ASPHALT   => refinery::OilProductType::ASPHALT,
            OilProductEnum::HEAVY     => refinery::OilProductType::HEAVY,
            OilProductEnum::LUBRICANT => refinery::OilProductType::LUBRICANT,
            OilProductEnum::OTHER     => refinery::OilProductType::OTHER,
        };

        let mut order = refinery::OrderForm::new();
        order.set_quantity(rust_form.quantity);
        order.set_product(product_parsed);

        order

    }
}
