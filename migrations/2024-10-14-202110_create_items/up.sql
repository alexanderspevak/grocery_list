CREATE TYPE product_unit AS ENUM ('g', 'kg', 'pc', 'l', 'ml');

CREATE TABLE items(
  id UUID PRIMARY KEY,
  product_id UUID NOT NULL,
  group_id UUID NOT NULL,
  unit product_unit DEFAULT 'pc',
  quantity decimal DEFAULT 1,
  CONSTRAINT fk_product FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
  CONSTRAINT fk_group FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
)
