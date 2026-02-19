CREATE TABLE payments_service_scheduled (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    schedule_service_information_id UUID NOT NULL,
    user_id UUID NOT NULL,
    provider VARCHAR NOT NULL, -- "asaas"
    provider_payment_id VARCHAR, -- ID of Asaas
    status VARCHAR NOT NULL, -- pending | paid | failed | expired
    cost DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
)
