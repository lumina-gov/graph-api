CREATE EXTENSION "uuid-ossp";

CREATE TYPE "assessment" AS ENUM ('UNKNOWN', 'FAIL', 'SOFT_PASS', 'PASS');

CREATE TYPE "unit_status" AS ENUM ('Completed', 'InProgress', 'NotStarted');

CREATE TABLE "public"."__diesel_schema_migrations" (
    "version" character varying(50) NOT NULL,
    "run_on" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("version")
);

CREATE TABLE "public"."applications" (
    "id" uuid NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "application" jsonb NOT NULL,
    "application_type" character varying NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."oauth_apps" (
    "client_id" character varying NOT NULL,
    "redirect_uris" character varying NOT NULL,
    "app_name" character varying NOT NULL,
    "client_secret" character varying,
    PRIMARY KEY ("client_id")
);

CREATE TABLE "public"."oauth_grants" (
    "user_id" uuid NOT NULL,
    "client_id" character varying NOT NULL,
    "scopes" character varying NOT NULL,
    "refresh_token" character varying NOT NULL,
    PRIMARY KEY ("user_id", "client_id")
);

CREATE TABLE "public"."question_assessments" (
    "id" uuid NOT NULL,
    "user_id" uuid NOT NULL,
    "course_slug" character varying NOT NULL,
    "unit_slug" character varying NOT NULL,
    "question_slug" character varying NOT NULL,
    "answer" character varying NOT NULL,
    "assessment" assessment NOT NULL,
    "feedback" character varying NOT NULL,
    "updated_at" timestamp with time zone NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX idx_assessment_questions_user_course_unit_question ON public.question_assessments USING btree (user_id, course_slug, unit_slug, question_slug);

ALTER TABLE ONLY "public"."question_assessments" ADD CONSTRAINT "question_assessments_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION;

CREATE TABLE "public"."unit_progress" (
    "id" uuid NOT NULL,
    "user_id" uuid NOT NULL,
    "unit_slug" character varying NOT NULL,
    "course_slug" character varying NOT NULL,
    "status" unit_status NOT NULL,
    "updated_at" timestamp with time zone NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

ALTER TABLE ONLY "public"."unit_progress" ADD CONSTRAINT "unit_progress_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION;

ALTER TABLE "public"."unit_progress" ADD CONSTRAINT "unique_user_unit_course" UNIQUE (user_id, unit_slug, course_slug);

CREATE TABLE "public"."users" (
    "id" uuid NOT NULL,
    "email" character varying NOT NULL,
    "joined" timestamp with time zone NOT NULL DEFAULT now(),
    "password" character varying NOT NULL,
    "first_name" character varying NOT NULL,
    "last_name" character varying NOT NULL,
    "calling_code" character varying NOT NULL,
    "country_code" character varying NOT NULL,
    "phone_number" character varying NOT NULL,
    "role" character varying,
    "referrer" uuid,
    PRIMARY KEY ("id")
);
