CREATE TABLE services_consultants(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consultant_id UUID NOT NULL,
    service_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    
    FOREIGN KEY (consultant_id) REFERENCES consultants (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    
    FOREIGN KEY (service_id) REFERENCES services (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);