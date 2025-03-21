CREATE TABLE IF NOT EXISTS merchant (
    id  TEXT PRIMARY KEY,
    name  TEXT DEFAULT '',
    premise  TEXT DEFAULT '',
    street  TEXT DEFAULT '',
    city  TEXT DEFAULT '',
    postcode TEXT DEFAULT '',
    county  TEXT DEFAULT '',
    country  char(2) DEFAULT 'GB'
);

INSERT INTO merchant VALUES ('merchant123', 'Test Merchant', 'Premise', 'Street', 'City', 'Postcode', 'County', 'GB');

CREATE SCHEMA IF NOT EXISTS account;
CREATE TABLE IF NOT EXISTS account.bankone (
    id INTEGER PRIMARY KEY,
    merchant_identification_value TEXT
);

INSERT INTO account.bankone VALUES (0, '123');

CREATE TABLE IF NOT EXISTS paymentroute (
    scheme TEXT,
    currency TEXT,
    merchant_id TEXT REFERENCES merchant,
    account_id INTEGER,
    acquirer TEXT,
    PRIMARY KEY (scheme, currency, merchant_id)
);

INSERT INTO paymentroute VALUES 
    ('VISA', 'GBP', 'merchant123', 0, 'bankone'),
    ('VISA', 'USD', 'merchant123', 0, 'bankone')
;

CREATE TABLE IF NOT EXISTS transaction (
    reference TEXT PRIMARY KEY,
    transaction_type TEXT NOT NULL,
    merchant_id TEXT REFERENCES merchant NOT NULL
);
