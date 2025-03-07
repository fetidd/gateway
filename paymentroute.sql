CREATE TYPE scheme AS ENUM ('VISA', 'NASTERCARD');
CREATE TYPE currency AS ENUM ('GBP', 'USD');
CREATE TYPE bank AS ENUM ('bankone', 'banktwo');

CREATE TABLE paymentroute (
    scheme scheme,
    currency currency,
    merchant_id varchar(255) REFERENCES merchant,
    account_id integer,
    bank bank,
    PRIMARY KEY (scheme, currency, merchant_id)
);

INSERT INTO paymentroute VALUES 
    ('VISA', 'GBP', 'merchant123', 0, 'bankone'),
    ('VISA', 'USD', 'merchant123', 0, 'bankone')
;

GRANT ALL ON paymentroute TO gwuser;
