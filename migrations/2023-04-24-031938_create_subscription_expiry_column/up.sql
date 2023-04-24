-- Your SQL goes here

ALTER TABLE users
ADD COLUMN subscription_expiry_date timestamp with time zone NULL;
