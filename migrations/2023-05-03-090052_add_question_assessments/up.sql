-- Your SQL goes here

CREATE TYPE Assessment AS ENUM ('PASS', 'SOFT_PASS', 'FAIL', 'UNKNOWN');

CREATE TABLE question_assessments (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) NOT NULL,
    course_slug character varying NOT NULL,
    unit_slug character varying NOT NULL,
    question_slug character varying NOT NULL,
    answer character varying NOT NULL,
    assessment Assessment NOT NULL,
    feedback character varying NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_assessment_questions_user_course_unit_question
    ON question_assessments (user_id, course_slug, unit_slug, question_slug);
