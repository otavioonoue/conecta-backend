CREATE TABLE consultant (
  id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name       VARCHAR(255) NOT NULL,
  email      VARCHAR(255) UNIQUE NOT NULL,
  phone      VARCHAR(13) UNIQUE NOT NULL,
  password   VARCHAR(255) NOT NULL,
  active     BOOLEAN NOT NULL DEFAULT true,
  service_id UUID,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL,
  
  FOREIGN KEY (service_id) REFERENCES services (id)
    ON UPDATE CASCADE
);