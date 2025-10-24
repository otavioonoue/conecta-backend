CREATE TABLE services (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name        VARCHAR(100) UNIQUE NOT NULL,
  travel_cost DECIMAL(10, 2) NOT NULL,
  created_at  TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE TABLE service_status (
  id   INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name VARCHAR(100) UNIQUE NOT NULL
);

INSERT INTO service_status (name) VALUES
('PENDENTE'),
('CONCLUÍDO'),
('CANCELADO')