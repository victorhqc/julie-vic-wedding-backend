table! {
    use use diesel::sql_types::*;
    use crate::attend_status_type::AttendStatusType;

    confirmed_users (user_id) {
        user_id -> Uuid,
        will_attend -> AttendStatusType,
        table_id -> Nullable<Uuid>,
        token_id -> Uuid,
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
    tokens (id) {
        id -> Uuid,
        token -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
joinable!(confirmed_users -> tokens (token_id));
joinable!(confirmed_users -> users (user_id));

allow_tables_to_appear_in_same_query!(confirmed_users, tables, tokens, users,);
