CREATE TABLE services_budgets_status (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE services_budgets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_scheduled_id UUID,
    service_cost DECIMAL(10, 2) NOT NULL,
    travel_cost DECIMAL(10, 2) NOT NULL,
    description VARCHAR(500),
    service_budget_status_id INT,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    
    FOREIGN KEY (service_scheduled_id) REFERENCES services_scheduled (id)
        ON UPDATE CASCADE,
    
    FOREIGN KEY (service_budget_status_id) REFERENCES services_budgets_status (id)
        ON UPDATE CASCADE
);

INSERT INTO services_budgets_status (name) VALUES 
('ACEITO'),
('DECLINADO')