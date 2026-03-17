CREATE TABLE service_steps (
    id SMALLINT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE service_information (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL,
    service_id      UUID NOT NULL,
    consultant_id   UUID,
    service_step_id SMALLINT NOT NULL,
    address_id      UUID NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    FOREIGN KEY (user_id) REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
      
    FOREIGN KEY (service_id) REFERENCES services (id)
        ON UPDATE CASCADE,
      
    FOREIGN KEY (consultant_id) REFERENCES consultants (id)
        ON UPDATE CASCADE,
    
    FOREIGN KEY (address_id) REFERENCES addresses (id)
        ON UPDATE CASCADE,
    
    FOREIGN KEY (service_step_id) REFERENCES service_steps (id)
);

INSERT INTO service_steps (id, name) VALUES 
(1, 'CANCELLED'),
(2, 'VISIT_SCHEDULED'),
(3, 'VISIT_PAYMENT_CREATED'),
(4, 'VISIT_PAYMENT_RECEIVED'),
(5, 'VISIT_CONFIRMED'),
(6, 'BUDGET_RECEIVED'),
(7, 'BUDGET_CONFIRMED'),
(8, 'BUDGET_DENIED'),
(9, 'BUDGET_PAYMENT_CREATED'),
(10, 'BUDGET_PAYMENT_RECEIVED'),
(11, 'SERVICE_ORDER_RECEIVED'),
(12, 'SERVICE_ORDER_SCHEDULE_CONFIRMED'),
(13, 'SERVICE_ORDER_SCHEDULE_DENIED'),
(14, 'SERVICE_ORDER_COMPLETED');


