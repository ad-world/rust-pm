// @generated automatically by Diesel CLI.

diesel::table! {
    vault (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
