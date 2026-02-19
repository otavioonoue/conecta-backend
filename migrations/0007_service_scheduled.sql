CREATE TABLE services_scheduled (
  id                     UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  service_information_id UUID,
  service_status_id      INT,
  description            VARCHAR(500),
  scheduled_at           TIMESTAMPTZ DEFAULT now() NOT NULL,
  scheduled_to           TIMESTAMPTZ NOT NULL,
  
  FOREIGN KEY (service_information_id) REFERENCES service_information (id)
    ON UPDATE CASCADE,
  
  FOREIGN KEY (service_status_id) REFERENCES service_status (id)
    ON UPDATE CASCADE
);