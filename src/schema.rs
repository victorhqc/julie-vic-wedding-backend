table! {
    confirmed_users (user_id) {
        user_id -> Uuid,
        table_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tables (id) {
        id -> Uuid,
        name -> Text,
        alias -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        last_name -> Nullable<Text>,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(confirmed_users -> tables (table_id));
joinable!(confirmed_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    confirmed_users,
    tables,
    users,
);
