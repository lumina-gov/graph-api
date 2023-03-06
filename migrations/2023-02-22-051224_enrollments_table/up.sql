CREATE TABLE enrollments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    course_id UUID NOT NULL REFERENCES courses(id),
    enrolled_date TIMESTAMP WITH TIME ZONE NOT NULL,
    UNIQUE (user_id, course_id)
);