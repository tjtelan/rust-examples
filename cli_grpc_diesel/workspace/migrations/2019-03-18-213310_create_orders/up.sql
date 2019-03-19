CREATE TYPE oil_products AS ENUM (
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
  product oil_products NOT NULL
);
