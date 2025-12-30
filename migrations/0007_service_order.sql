CREATE TABLE services_order (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_budget_id UUID,
    final_cost DECIMAL(10, 2) NOT NULL,
    description VARCHAR(500) NOT NULL,
    scheduled_at TIMESTAMP DEFAULT NOW() NOT NULL,
    scheduled_to TIMESTAMP NOT NULL,
    
    FOREIGN KEY (service_budget_id) REFERENCES services_budgets (id)
        ON UPDATE CASCADE
);