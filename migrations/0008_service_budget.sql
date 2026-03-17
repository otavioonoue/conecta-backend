CREATE TABLE services_budgets_status (
    id SMALLINT PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE services_budgets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_information_id UUID,
    service_cost DECIMAL(10, 2) NOT NULL,
    travel_cost DECIMAL(10, 2) NOT NULL,
    description VARCHAR(500),
    service_budget_status_id SMALLINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    
    FOREIGN KEY (service_information_id) REFERENCES service_information (id)
      ON UPDATE CASCADE,
    
    FOREIGN KEY (service_budget_status_id) REFERENCES services_budgets_status (id)
        ON UPDATE CASCADE
);

INSERT INTO services_budgets_status (id, name) VALUES 
(1, 'PENDING'),
(2, 'ACCEPTED'),
(3, 'REJECTED')