CREATE TABLE bankone_account (
    id  integer PRIMARY KEY,
    merchant_id varchar(255)
);

CREATE TABLE banktwo_account (
    id  integer PRIMARY KEY,
    merchant_id varchar(255)
);

INSERT INTO bankone_account  VALUES (0, 'merchant123');
INSERT INTO banktwo_account  VALUES (1, 'merchant123');

GRANT ALL ON bankone_account TO gwuser;
GRANT ALL ON banktwo_account TO gwuser;
