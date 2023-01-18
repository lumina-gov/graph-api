// @generated automatically by Diesel CLI.

diesel::table! {
    applications (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        application -> Jsonb,
    }
}

diesel::table! {
    course_progress (id) {
        id -> Int4,
        course_id -> Uuid,
        user_id -> Uuid,
        credits -> Int4,
    }
}

diesel::table! {
    course_to_creator (id) {
        id -> Int8,
        course -> Uuid,
        creator -> Uuid,
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
    creators (id) {
        first_name -> Varchar,
        last_name -> Varchar,
        id -> Uuid,
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
    }
}

diesel::joinable!(course_progress -> courses (course_id));
diesel::joinable!(course_progress -> users (user_id));
diesel::joinable!(course_to_creator -> courses (course));
diesel::joinable!(course_to_creator -> creators (creator));
diesel::joinable!(units -> courses (course_id));

diesel::allow_tables_to_appear_in_same_query!(
    applications,
    course_progress,
    course_to_creator,
    courses,
    creators,
    units,
    users,
);
