CREATE TABLE passwords (
  id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  nonce TEXT NOT NULL,
  website INT NOT NULL,
  account varchar(36) NOT NULL,
  FOREIGN KEY (website) REFERENCES websites(id),
  FOREIGN KEY (account) REFERENCES accounts(id)
)
