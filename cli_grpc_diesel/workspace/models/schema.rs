#[derive(Debug,DbEnum)]
pub enum OilProduct {
  GASOLINE,
  JETFUEL,
  DIESEL,
  ASPHALT,
  HEAVY,
  LUBRICANT,
  OTHER,
}

table! {
    use diesel::sql_types::Integer;
    use super::OilProductMapping;

    orders {
        id -> Integer,
        quantity -> Integer,
        product_type -> OilProductMapping,
    }
}
