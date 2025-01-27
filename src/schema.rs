// @generated automatically by Diesel CLI.

diesel::table! {
    bank_accounts (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        account_number -> Integer,
        agency -> Integer,
        bank_name -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    email_verification_tokens (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        token -> Text,
        expires_at -> Timestamp,
        used -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_actived -> Nullable<Bool>,
    }
}

diesel::joinable!(bank_accounts -> users (user_id));
diesel::joinable!(email_verification_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bank_accounts,
    email_verification_tokens,
    users,
);
