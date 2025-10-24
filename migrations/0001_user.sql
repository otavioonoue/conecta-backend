CREATE TABLE addresses (
  id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  cep          VARCHAR(8) NOT NULL,
  number       VARCHAR(6) NOT NULL,
  street       VARCHAR(255),
  neighborhood VARCHAR(255),
  city         VARCHAR(100),
  state        VARCHAR(100),
  country      VARCHAR(100)
);

CREATE TABLE users (
  id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name       VARCHAR(255) NOT NULL,
  email      VARCHAR(255) UNIQUE NOT NULL,
  phone      VARCHAR(13) UNIQUE NOT NULL,
  cpf        VARCHAR(11) UNIQUE NOT NULL,
  active     BOOLEAN NOT NULL DEFAULT true,
  address_id UUID,
  password   VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL,
  
  FOREIGN KEY (address_id) REFERENCES addresses (id)
    ON DELETE CASCADE 
    ON UPDATE CASCADE
);