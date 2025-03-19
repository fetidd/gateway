CREATE SCHEMA IF NOT EXISTS account;

-- CREATE USER gwuser WITH PASSWORD 'gwpass';
CREATE TABLE IF NOT EXISTS account.merchant (
    id  varchar(255) PRIMARY KEY,
    name  varchar(255) DEFAULT '',
    premise  varchar(255) DEFAULT '',
    street  varchar(255) DEFAULT '',
    city  varchar(255) DEFAULT '',
    postcode varchar(255) DEFAULT '',
    county  varchar(255) DEFAULT '',
    country  char(2) DEFAULT 'GB'
);

INSERT INTO account.merchant VALUES ('merchant123', 'Test Merchant', 'Premise', 'Street', 'City', 'Postcode', 'County', 'GB');

-- GRANT ALL ON account.merchant TO gwuser;
CREATE TABLE IF NOT EXISTS account.bankone (
    id  integer PRIMARY KEY,
    merchant_identification_value varchar(255)
);

CREATE TABLE IF NOT EXISTS account.banktwo (
    id  integer PRIMARY KEY,
    banktwo_merchant_id varchar(255)
);

INSERT INTO account.bankone VALUES (0, 'merchant123');
-- INSERT INTO account.banktwo VALUES (1, 'merchant123');

-- GRANT ALL ON account.bankone TO gwuser;
-- GRANT ALL ON account.banktwo TO gwuser;
-- CREATE TYPE scheme AS ENUM ('VISA', 'MASTERCARD');
-- CREATE TYPE currency AS ENUM ('GBP', 'USD');
-- CREATE TYPE acquirer AS ENUM ('bankone', 'banktwo');

CREATE TABLE IF NOT EXISTS account.paymentroute (
    scheme TEXT,
    currency TEXT,
    merchant_id varchar(255) REFERENCES account.merchant,
    account_id integer,
    acquirer TEXT,
    PRIMARY KEY (scheme, currency, merchant_id)
);

INSERT INTO account.paymentroute VALUES 
    ('VISA', 'GBP', 'merchant123', 0, 'bankone'),
    ('VISA', 'USD', 'merchant123', 0, 'bankone')
;

-- GRANT ALL ON account.paymentroute TO gwuser;


CREATE SCHEMA IF NOT EXISTS transaction;

CREATE TABLE IF NOT EXISTS transaction.bankone (
    reference TEXT PRIMARY KEY,
    transaction_type TEXT NOT NULL,
    merchant_id varchar(255) REFERENCES account.merchant NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    card_scheme TEXT default '',
    encrypted_pan TEXT,
    masked_pan TEXT,
    expiry_date TEXT DEFAULT '',
    billing_name TEXT DEFAULT '',
    billing_premise TEXT DEFAULT '',
    billing_street TEXT DEFAULT '',
    billing_city TEXT DEFAULT '',
    billing_country TEXT DEFAULT '',
    billing_county TEXT DEFAULT 'GB',
    customer_name TEXT DEFAULT '',
    customer_premise TEXT DEFAULT '',
    customer_street TEXT DEFAULT '',
    customer_city TEXT DEFAULT '',
    customer_country TEXT DEFAULT '',
    customer_county TEXT DEFAULT 'GB',

    merchant_identification_value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS transaction.banktwo (
    reference TEXT PRIMARY KEY,
    transaction_type TEXT NOT NULL,
    merchant_id varchar(255) REFERENCES account.merchant NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    card_scheme TEXT default '',
    encrypted_pan TEXT,
    masked_pan TEXT,
    expiry_date TEXT DEFAULT '',
    billing_name TEXT DEFAULT '',
    billing_premise TEXT DEFAULT '',
    billing_street TEXT DEFAULT '',
    billing_city TEXT DEFAULT '',
    billing_country TEXT DEFAULT '',
    billing_county TEXT DEFAULT 'GB',
    customer_name TEXT DEFAULT '',
    customer_premise TEXT DEFAULT '',
    customer_street TEXT DEFAULT '',
    customer_city TEXT DEFAULT '',
    customer_country TEXT DEFAULT '',
    customer_county TEXT DEFAULT 'GB',
    
    banktwo_merchant_id TEXT NOT NULL
);

