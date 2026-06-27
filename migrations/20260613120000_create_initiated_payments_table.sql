CREATE TABLE initiated_payments (
    id                  UUID            PRIMARY KEY DEFAULT uuid_generate_v4(),
    payment_reference   VARCHAR(100)    NOT NULL REFERENCES payments(reference),
    amount              NUMERIC(15, 2)  NOT NULL,
    gateway             VARCHAR(255)    NOT NULL,
    gateway_reference   VARCHAR(255),
    checkout_url        VARCHAR(1000),
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE (payment_reference, gateway)
);

CREATE INDEX idx_initiated_payments_reference ON initiated_payments(payment_reference);
CREATE INDEX idx_initiated_payments_gateway   ON initiated_payments(gateway);
