CREATE TABLE services_order_status (
    id SMALLINT PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE services_order (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_information_id UUID,
    final_cost DECIMAL(10, 2) NOT NULL,
    description VARCHAR(500) NOT NULL,
    service_order_status_id SMALLINT NOT NULL,
    scheduled_to TIMESTAMPTZ NOT NULL,
    scheduled_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    
    FOREIGN KEY (service_information_id) REFERENCES service_information (id)
        ON UPDATE CASCADE,
      
    FOREIGN KEY (service_order_status_id) REFERENCES services_order_status (id)
        ON UPDATE CASCADE
);

INSERT INTO services_order_status (id, name) VALUES 
(1, 'PENDING'),
(2, 'ACCEPTED'),
(3, 'REJECTED'),
(4, 'COMPLETED')