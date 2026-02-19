CREATE TABLE services_order (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_information_id UUID,
    final_cost DECIMAL(10, 2) NOT NULL,
    description VARCHAR(500) NOT NULL,
    scheduled_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    scheduled_to TIMESTAMPTZ NOT NULL,
    
    FOREIGN KEY (service_information_id) REFERENCES service_information (id)
      ON UPDATE CASCADE
);