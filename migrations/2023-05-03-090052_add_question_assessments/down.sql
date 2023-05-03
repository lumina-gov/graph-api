-- This file should undo anything in `up.sql`

DROP INDEX idx_assessment_questions_user_course_unit_question;
DROP TABLE question_assessments;
DROP TYPE Assessment;
