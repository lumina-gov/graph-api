CREATE EXTENSION "uuid-ossp";

CREATE TYPE "assessment" AS ENUM ('PASS','SOFT_PASS','FAIL','UNKNOWN');

CREATE TYPE "unit_status" AS ENUM ('NotStarted','InProgress','Completed' );

CREATE TABLE "public"."users" (
    "id" uuid PRIMARY KEY NOT NULL,
    "email" character varying NOT NULL,
    "joined" timestamp with time zone NOT NULL DEFAULT now(),
    "password" character varying NOT NULL,
    "first_name" character varying NOT NULL,
    "last_name" character varying NOT NULL,
    "calling_code" character varying NOT NULL,
    "country_code" character varying NOT NULL,
    "phone_number" character varying NOT NULL,
    "role" character varying,
    "referrer" uuid REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE SET NULL,
    "stripe_customer_id" character varying
);

CREATE UNIQUE INDEX index_users_email ON public.users USING btree (email);

CREATE TABLE "public"."applications" (
    "id" uuid PRIMARY KEY NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "application" jsonb NOT NULL,
    "application_type" character varying NOT NULL
);

CREATE TABLE "public"."oauth_apps" (
    "client_id" character varying PRIMARY KEY NOT NULL,
    "redirect_uris" character varying NOT NULL,
    "app_name" character varying NOT NULL,
    "client_secret" character varying
);

CREATE TABLE "public"."oauth_grants" (
    "user_id" uuid NOT NULL,
    "client_id" character varying NOT NULL,
    "scopes" character varying NOT NULL,
    "refresh_token" character varying NOT NULL,
    PRIMARY KEY ("user_id", "client_id")
);

CREATE TABLE "public"."question_assessments" (
    "id" uuid PRIMARY KEY NOT NULL,
    "user_id" uuid REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
    "course_slug" character varying NOT NULL,
    "unit_slug" character varying NOT NULL,
    "question_slug" character varying NOT NULL,
    "answer" character varying NOT NULL,
    "assessment" assessment NOT NULL,
    "feedback" character varying NOT NULL,
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX index_assessment_questions_user_course_unit_question ON public.question_assessments USING btree (user_id, course_slug, unit_slug, question_slug);

CREATE TABLE "public"."unit_progress" (
    "id" uuid PRIMARY KEY NOT NULL,
    "user_id" uuid REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
    "unit_slug" character varying NOT NULL,
    "course_slug" character varying NOT NULL,
    "status" unit_status NOT NULL,
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

ALTER TABLE "public"."unit_progress" ADD CONSTRAINT "unique_user_unit_course" UNIQUE (user_id, unit_slug, course_slug);

CREATE TABLE "public"."password_reset_tokens" (
    "id" uuid PRIMARY KEY NOT NULL,
    "user_id" uuid NOT NULL REFERENCES "public"."users"(id) ON DELETE CASCADE,
    "expires_at" timestamp with time zone NOT NULL
);
