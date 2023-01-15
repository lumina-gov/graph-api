// @generated automatically by Diesel CLI.

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
    }
}

diesel::joinable!(course_to_creator -> courses (course));
diesel::joinable!(course_to_creator -> creators (creator));
diesel::joinable!(units -> courses (course_id));

diesel::allow_tables_to_appear_in_same_query!(course_to_creator, courses, creators, units,);
