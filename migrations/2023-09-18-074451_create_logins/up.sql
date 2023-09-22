CREATE TABLE logins (
  token varchar(36) NOT NULL PRIMARY KEY,
  account varchar(36) NOT NULL,
  FOREIGN KEY (account) REFERENCES accounts(id)
)
