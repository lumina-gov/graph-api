--
-- Name: applications; Type: TABLE; Schema: public; Owner: supabase_admin
--

CREATE TABLE applications (
    id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    application jsonb NOT NULL,
    application_type character varying NOT NULL
);


--
-- Name: courses; Type: TABLE; Schema: public; Owner: supabase_admin
--

CREATE TABLE courses (
    id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    name character varying DEFAULT ''::character varying NOT NULL,
    slug text NOT NULL
);


--
-- Name: units; Type: TABLE; Schema: public; Owner: supabase_admin
--

CREATE TABLE units (
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    name character varying DEFAULT ''::character varying NOT NULL,
    id uuid DEFAULT extensions.uuid_generate_v4() NOT NULL,
    parent_unit uuid,
    course_id uuid NOT NULL,
    slug text NOT NULL
);


--
-- Name: users; Type: TABLE; Schema: public; Owner: supabase_admin
--

CREATE TABLE users (
    id uuid PRIMARY KEY,
    email character varying NOT NULL,
    joined timestamp with time zone DEFAULT now() NOT NULL,
    password character varying NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    calling_code character varying NOT NULL,
    country_code character varying NOT NULL,
    phone_number character varying NOT NULL,
    role character varying,
    object_id character varying,
    referrer uuid,
    referrer_mongo character varying
);
