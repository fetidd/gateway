CREATE TABLE IF NOT EXISTS merchant (
    id  varchar(255) PRIMARY KEY,
    name  varchar(255) DEFAULT '',
    premise  varchar(255) DEFAULT '',
    street  varchar(255) DEFAULT '',
    city  varchar(255) DEFAULT '',
    postcode varchar(255) DEFAULT '',
    county  varchar(255) DEFAULT '',
    country  char(2) DEFAULT 'GB'
);

INSERT INTO merchant VALUES ('merchant123', 'Test Merchant', 'Premise', 'Street', 'City', 'Postcode', 'County', 'GB');

GRANT ALL ON merchant TO gwuser;
