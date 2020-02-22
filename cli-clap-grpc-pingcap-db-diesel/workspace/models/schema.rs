use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

table! {
    use diesel::sql_types::{Integer, Timestamp};
    use super::OilProductTypeDieselWrapper;

    // Note to self: This will create a struct `orders` that we can import to pass to
    // Diesel's `table_name` macro
    orders {
        id -> Integer,
        quantity -> Integer,
        product_type -> OilProductTypeDieselWrapper,
        received_time -> Timestamp,
    }
}

// Note to self. These names are kind of terrible.
// This was modeled after Diesel's tests for using custom types
// https://github.com/diesel-rs/diesel/blob/v1.3.1/diesel_tests/tests/custom_types.rs
// We can derive SqlType because we implement ToSql/FromSql
#[derive(SqlType, Debug)]
#[postgres(type_name = "oil_product")]
pub struct OilProductTypeDieselWrapper;

#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "OilProductTypeDieselWrapper"]
pub enum OilProductEnum {
    GASOLINE,
    JETFUEL,
    DIESEL,
    ASPHALT,
    HEAVY,
    LUBRICANT,
    OTHER,
}

impl ToSql<OilProductTypeDieselWrapper, Pg> for OilProductEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            OilProductEnum::GASOLINE => out.write_all(b"GASOLINE")?,
            OilProductEnum::JETFUEL => out.write_all(b"JETFUEL")?,
            OilProductEnum::DIESEL => out.write_all(b"DIESEL")?,
            OilProductEnum::ASPHALT => out.write_all(b"ASPHALT")?,
            OilProductEnum::HEAVY => out.write_all(b"HEAVY")?,
            OilProductEnum::LUBRICANT => out.write_all(b"LUBRICANT")?,
            OilProductEnum::OTHER => out.write_all(b"OTHER")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<OilProductTypeDieselWrapper, Pg> for OilProductEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"GASOLINE" => Ok(OilProductEnum::GASOLINE),
            b"JETFUEL" => Ok(OilProductEnum::JETFUEL),
            b"DIESEL" => Ok(OilProductEnum::DIESEL),
            b"ASPHALT" => Ok(OilProductEnum::ASPHALT),
            b"HEAVY" => Ok(OilProductEnum::HEAVY),
            b"LUBRICANT" => Ok(OilProductEnum::LUBRICANT),
            b"OTHER" => Ok(OilProductEnum::OTHER),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
