use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::pg::Pg;

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

pub struct OrderForm {
    pub quantity : i32,
    pub product_type : OilProductEnum,
}

impl OrderForm {
    // This is for taking in command line input from a client
    pub fn new(q : i32, p : String) -> Self {

        //let quantity = matches.value_of("quantity").unwrap().parse::<i32>().expect("Quantity should be a number");
        println!("# of barrels to ship to refinery: {:?}", q);

        // Convert product to enum
        let product = match p.as_ref() {
            "gasoline" => OilProductEnum::GASOLINE,
            "jetfuel" => OilProductEnum::JETFUEL,
            "diesel" => OilProductEnum::DIESEL,
            "asphalt" => OilProductEnum::ASPHALT,
            "heavy" => OilProductEnum::HEAVY,
            "lubricant" => OilProductEnum::LUBRICANT,
            _ => OilProductEnum::OTHER,
        };
         
        println!("To be refined into: {:?}", product);

        OrderForm {
            quantity : q,
            product_type : product,
        }
    }
}

// Convert from the protos to our type
impl From<refinery::OrderForm> for OrderForm {
    fn from(proto_form : refinery::OrderForm) -> Self {
        let product_parsed = match proto_form.get_product() {
            refinery::OilProductType::GASOLINE => OilProductEnum::GASOLINE,
            refinery::OilProductType::JETFUEL => OilProductEnum::JETFUEL,
            refinery::OilProductType::DIESEL => OilProductEnum::DIESEL,
            refinery::OilProductType::ASPHALT => OilProductEnum::ASPHALT,
            refinery::OilProductType::HEAVY => OilProductEnum::HEAVY,
            refinery::OilProductType::LUBRICANT => OilProductEnum::LUBRICANT,
            refinery::OilProductType::OTHER => OilProductEnum::OTHER,
        };
 
        OrderForm {
            quantity : proto_form.get_quantity(),
            product_type : product_parsed,
        }
    }
}

// The reciprocal form to
impl From<OrderForm> for refinery::OrderForm {
    fn from(rust_form : OrderForm) -> Self {
        // We're going to make an order
        // Build our data payload.
        let mut order = refinery::OrderForm::new();
        order.set_quantity(rust_form.quantity);


        let product = match rust_form.product_type {
            OilProductEnum::GASOLINE => refinery::OilProductType::GASOLINE,
            OilProductEnum::JETFUEL => refinery::OilProductType::JETFUEL,
            OilProductEnum::DIESEL => refinery::OilProductType::DIESEL,
            OilProductEnum::ASPHALT => refinery::OilProductType::ASPHALT,
            OilProductEnum::HEAVY => refinery::OilProductType::HEAVY,
            OilProductEnum::LUBRICANT => refinery::OilProductType::LUBRICANT,
            OilProductEnum::OTHER => refinery::OilProductType::OTHER,
        };

        order.set_product(product);

        order


    }
}
