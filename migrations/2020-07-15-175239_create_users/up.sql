-- Your SQL goes here
CREATE TABLE user (
  id CHAR(36) NOT NULL PRIMARY KEY,
  username NVARCHAR(255) NOT NULL,
  serverside_processing BOOLEAN NOT NULL,
  last_login_time TIMESTAMP NOT NULL,
  session_data CHAR(36),
  session_expiration TIMESTAMP NULL
)