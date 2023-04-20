-- This file should undo anything in `up.sql`
ALTER TABLE users
ADD COLUMN referrer_mongo varchar;

ALTER TABLE users
ADD COLUMN object_id varchar;