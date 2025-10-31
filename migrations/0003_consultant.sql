CREATE TABLE consultants (
  id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name       VARCHAR(255) NOT NULL,
  email      VARCHAR(255) UNIQUE NOT NULL,
  phone      VARCHAR(13) UNIQUE NOT NULL,
  password   VARCHAR(255) NOT NULL,
  active     BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
);