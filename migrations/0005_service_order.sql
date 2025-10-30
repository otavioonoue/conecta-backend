CREATE TABLE services_order (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_scheduled_id UUID,
    service_cost DECIMAL(10, 2) NOT NULL,
    description VARCHAR(500) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    
    FOREIGN KEY (service_scheduled_id) REFERENCES services_scheduled (id)
        ON UPDATE CASCADE
);