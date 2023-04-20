-- Your SQL goes here

BEGIN;

ALTER TABLE users
DROP COLUMN referrer_mongo;

ALTER TABLE users
DROP COLUMN object_id;

COMMIT;