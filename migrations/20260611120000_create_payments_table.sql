 
CREATE TABLE payments (
    id                  UUID            PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id             UUID            NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    wallet_id           UUID            NOT NULL,
    amount              NUMERIC(15, 2)  NOT NULL,
    currency            CHAR(3)         NOT NULL,
    reference           VARCHAR(100)    NOT NULL UNIQUE,
    status              VARCHAR(255)    NOT NULL DEFAULT 'Pending',
    description         VARCHAR(255),
    gateway_reference   VARCHAR(255),
    gateway             VARCHAR(255),
    paid_at             TIMESTAMPTZ,
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ    
);

CREATE INDEX idx_payments_user_id   ON payments(user_id);
CREATE INDEX idx_payments_wallet_id ON payments(wallet_id);
CREATE INDEX idx_payments_status    ON payments(status);
