// @generated automatically by Diesel CLI.

diesel::table! {
    applications (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        application -> Jsonb,
        application_type -> Varchar,
    }
}

diesel::table! {
    bank (id) {
        created_at -> Nullable<Timestamptz>,
        name -> Text,
        account_number -> Text,
        id -> Int8,
    }
}

diesel::table! {
    courses (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        name -> Varchar,
        slug -> Text,
    }
}

diesel::table! {
    enrollments (id) {
        id -> Uuid,
        user_id -> Uuid,
        course_id -> Uuid,
        enrolled_date -> Timestamptz,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        amount -> Nullable<Numeric>,
        from -> Nullable<Int8>,
        to -> Nullable<Int8>,
    }
}

diesel::table! {
    units (id) {
        created_at -> Timestamptz,
        name -> Varchar,
        id -> Uuid,
        parent_unit -> Nullable<Uuid>,
        course_id -> Uuid,
        slug -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        joined -> Timestamptz,
        password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        calling_code -> Varchar,
        country_code -> Varchar,
        phone_number -> Varchar,
        role -> Nullable<Varchar>,
        object_id -> Nullable<Varchar>,
        referrer -> Nullable<Uuid>,
        referrer_mongo -> Nullable<Varchar>,
        stripe_customer_id -> Nullable<Varchar>,
    }
}

diesel::joinable!(enrollments -> courses (course_id));
diesel::joinable!(enrollments -> users (user_id));
diesel::joinable!(units -> courses (course_id));

diesel::allow_tables_to_appear_in_same_query!(
    applications,
    bank,
    courses,
    enrollments,
    transactions,
    units,
    users,
);
