// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Uuid,
        owner -> Uuid,
        label -> Text,
        content -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        login -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    notes,
    users,
);
