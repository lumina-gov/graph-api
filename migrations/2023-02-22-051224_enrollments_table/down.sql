-- Revert foreign key constraints
ALTER TABLE enrollments DROP CONSTRAINT enrollments_user_id_fkey;
ALTER TABLE enrollments DROP CONSTRAINT enrollments_course_id_fkey;

-- Drop enrollments table
DROP TABLE enrollments;