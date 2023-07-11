-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE verify_tx (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  to_address TEXT,
  from_address TEXT,
  tx_hash TEXT,
  verify_status INTEGER
) CHARACTER SET utf8mb4;
