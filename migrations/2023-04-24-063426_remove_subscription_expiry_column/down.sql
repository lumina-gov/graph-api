-- This file should undo anything in `up.sql`
ALTER TABLE users
ADD COLUMN subscription_expiry_date timestamp with time zone NULL;