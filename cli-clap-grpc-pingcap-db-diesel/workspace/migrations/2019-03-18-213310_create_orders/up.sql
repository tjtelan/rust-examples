CREATE TYPE oil_product AS ENUM (
  'GASOLINE',
  'JETFUEL',
  'DIESEL',
  'ASPHALT',
  'HEAVY',
  'LUBRICANT',
  'OTHER'
);

CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  quantity INTEGER NOT NULL,
  product_type oil_product NOT NULL,
  received_time TIMESTAMP NOT NULL
);
