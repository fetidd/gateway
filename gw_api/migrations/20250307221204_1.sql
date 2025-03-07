-- Add migration script here
CREATE USER gwuser WITH PASSWORD 'gwpass';
CREATE TABLE account.merchant (
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

GRANT ALL ON account.merchant TO gwuser;
CREATE TABLE account.bankone (
    id  integer PRIMARY KEY,
    merchant_id varchar(255)
);

CREATE TABLE account.banktwo (
    id  integer PRIMARY KEY,
    merchant_id varchar(255)
);

INSERT INTO account.bankone  VALUES (0, 'merchant123');
INSERT INTO account.banktwo  VALUES (1, 'merchant123');

GRANT ALL ON account.bankone TO gwuser;
GRANT ALL ON account.banktwo TO gwuser;
CREATE TYPE account.scheme AS ENUM ('VISA', 'NASTERCARD');
CREATE TYPE account.currency AS ENUM ('GBP', 'USD');
CREATE TYPE account.bank AS ENUM ('bankone', 'banktwo');

CREATE TABLE account.paymentroute (
    scheme scheme,
    currency currency,
    merchant_id varchar(255) REFERENCES account.merchant,
    account_id integer,
    bank bank,
    PRIMARY KEY (scheme, currency, merchant_id)
);

INSERT INTO account.paymentroute VALUES 
    ('VISA', 'GBP', 'merchant123', 0, 'bankone'),
    ('VISA', 'USD', 'merchant123', 0, 'bankone')
;

GRANT ALL ON account.paymentroute TO gwuser;
