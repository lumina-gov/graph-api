-- Delete existing tables
DROP TABLE IF EXISTS enrollments;
DROP TABLE IF EXISTS courses;
DROP TABLE IF EXISTS units;

-- Create unit_status enum
CREATE TYPE unit_status AS ENUM ('NotStarted', 'InProgress', 'Completed');

-- Create unit_progress table with foreign key constraint
CREATE TABLE unit_progress (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    unit_slug character varying NOT NULL,
    course_slug character varying NOT NULL,
    status unit_status NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT unique_user_unit_course UNIQUE (user_id, unit_slug, course_slug)
);