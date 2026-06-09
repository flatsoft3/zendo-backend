-- Add migration script here
--add uuid generation  extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; 

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    first_name VARCHAR(100) NOT NULL,
    middle_name VARCHAR(100),
    last_name VARCHAR(100) NOT NULL,
    phone_number VARCHAR(50) NOT NULL,
    company_name VARCHAR(100),
    rc_number VARCHAR(50),
    tax_id VARCHAR(50),
    company_address VARCHAR(255),
    password VARCHAR(255) NOT NULL,
    password_reset_token VARCHAR(255),
    kyc_tier SMALLINT NOT NULL DEFAULT 0,
    kyc_verified_at TIMESTAMPTZ NULL,
    account_status VARCHAR(50) NOT NULL DEFAULT 'active',
    email_verified_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);