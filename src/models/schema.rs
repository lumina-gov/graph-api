// @generated automatically by Diesel CLI.

diesel::table! {
    course_to_creator (id) {
        id -> Int8,
        course -> Uuid,
        creator -> Uuid,
    }
}

diesel::table! {
    course_to_units (id) {
        id -> Int8,
        course -> Uuid,
        unit -> Uuid,
    }
}

diesel::table! {
    courses (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        name -> Varchar,
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
        created_at -> Nullable<Timestamptz>,
        name -> Nullable<Varchar>,
        id -> Uuid,
        parent_unit -> Nullable<Uuid>,
    }
}

diesel::joinable!(course_to_creator -> courses (course));
diesel::joinable!(course_to_creator -> creators (creator));
diesel::joinable!(course_to_units -> courses (course));
diesel::joinable!(course_to_units -> units (unit));

diesel::allow_tables_to_appear_in_same_query!(
    course_to_creator,
    course_to_units,
    courses,
    creators,
    units,
);
