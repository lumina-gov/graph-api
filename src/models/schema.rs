// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "unit_status"))]
    pub struct UnitStatus;
}

diesel::table! {
    applications (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        application -> Jsonb,
        application_type -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UnitStatus;

    unit_progress (id) {
        id -> Uuid,
        user_id -> Uuid,
        unit_slug -> Varchar,
        course_slug -> Varchar,
        status -> UnitStatus,
        updated_at -> Timestamptz,
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
        referrer -> Nullable<Uuid>,
        stripe_customer_id -> Nullable<Varchar>,
    }
}

diesel::joinable!(unit_progress -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    applications,
    unit_progress,
    users,
);
