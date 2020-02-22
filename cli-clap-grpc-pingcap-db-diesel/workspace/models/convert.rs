use crate::orders::{Order, OrderForm};
use crate::schema::OilProductEnum;
use chrono::NaiveDateTime;
use protos::refinery;

impl From<refinery::OilProductType> for OilProductEnum {
    fn from(proto_form: refinery::OilProductType) -> Self {
        match proto_form {
            refinery::OilProductType::GASOLINE => OilProductEnum::GASOLINE,
            refinery::OilProductType::JETFUEL => OilProductEnum::JETFUEL,
            refinery::OilProductType::DIESEL => OilProductEnum::DIESEL,
            refinery::OilProductType::ASPHALT => OilProductEnum::ASPHALT,
            refinery::OilProductType::HEAVY => OilProductEnum::HEAVY,
            refinery::OilProductType::LUBRICANT => OilProductEnum::LUBRICANT,
            refinery::OilProductType::OTHER => OilProductEnum::OTHER,
        }
    }
}

impl From<OilProductEnum> for refinery::OilProductType {
    fn from(rust_form: OilProductEnum) -> Self {
        match rust_form {
            OilProductEnum::GASOLINE => refinery::OilProductType::GASOLINE,
            OilProductEnum::JETFUEL => refinery::OilProductType::JETFUEL,
            OilProductEnum::DIESEL => refinery::OilProductType::DIESEL,
            OilProductEnum::ASPHALT => refinery::OilProductType::ASPHALT,
            OilProductEnum::HEAVY => refinery::OilProductType::HEAVY,
            OilProductEnum::LUBRICANT => refinery::OilProductType::LUBRICANT,
            OilProductEnum::OTHER => refinery::OilProductType::OTHER,
        }
    }
}

// Convert from the protos to our type
impl From<refinery::OrderForm> for OrderForm {
    fn from(proto_form: refinery::OrderForm) -> Self {
        OrderForm {
            quantity: proto_form.get_quantity(),
            product_type: OilProductEnum::from(proto_form.get_product()),
        }
    }
}

// Convert from our type to the proto
impl From<OrderForm> for refinery::OrderForm {
    fn from(rust_form: OrderForm) -> Self {
        let mut order = refinery::OrderForm::new();

        order.set_quantity(rust_form.quantity);
        order.set_product(refinery::OilProductType::from(rust_form.product_type));
        order
    }
}

// Convert for passing query results to client
impl From<Order> for refinery::OrderRecord {
    fn from(rust_form: Order) -> Self {
        let mut proto_timestamp = protobuf::well_known_types::Timestamp::new();
        proto_timestamp.set_seconds(rust_form.received_time.timestamp());
        proto_timestamp.set_nanos(rust_form.received_time.timestamp_nanos() as i32);

        let mut proto_form = refinery::OrderRecord::new();
        proto_form.set_id(rust_form.id);
        proto_form.set_quantity(rust_form.quantity);
        proto_form.set_product(refinery::OilProductType::from(rust_form.product_type));
        proto_form.set_received_time(proto_timestamp);
        proto_form
    }
}

impl From<refinery::OrderRecord> for Order {
    fn from(proto_form: refinery::OrderRecord) -> Self {
        Order {
            id: proto_form.get_id(),
            quantity: proto_form.get_quantity(),
            product_type: OilProductEnum::from(proto_form.get_product()),
            received_time: NaiveDateTime::from_timestamp(
                proto_form.get_received_time().get_seconds(),
                proto_form.get_received_time().get_nanos() as u32,
            ),
        }
    }
}
