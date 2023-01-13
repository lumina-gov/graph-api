// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        published -> Nullable<Bool>,
        title -> Nullable<Text>,
        body -> Nullable<Text>,
    }
}
