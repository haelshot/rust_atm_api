-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE app_users
(
id  uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
username varchar(255) NOT NULL,
firstname varchar(255) NOT NULL,
lastname varchar(255) NOT NULL,
email varchar(255) NOT NULL,
password varchar(255) NOT NULL,
is_active BOOLEAN NOT NULL DEFAULT FALSE
)