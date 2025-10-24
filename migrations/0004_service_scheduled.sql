CREATE TABLE service_scheduled (
  id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id           UUID,
  consultant_id     UUID,
  service_status_id INT,
  scheduled_at      TIMESTAMP DEFAULT NOW() NOT NULL,
  scheduled_to      TIMESTAMP NOT NULL,
  
  FOREIGN KEY (user_id) REFERENCES users (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  
  FOREIGN KEY (service_status_id) REFERENCES service_status (id)
    ON UPDATE CASCADE
);