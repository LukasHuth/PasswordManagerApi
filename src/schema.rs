// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        #[max_length = 36]
        id -> Varchar,
        username -> Nullable<Text>,
        #[max_length = 64]
        password -> Nullable<Char>,
    }
}

diesel::table! {
    logins (token) {
        #[max_length = 36]
        token -> Varchar,
        #[max_length = 36]
        account -> Varchar,
    }
}

diesel::table! {
    passwords (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        nonce -> Text,
        website -> Integer,
        #[max_length = 36]
        account -> Varchar,
    }
}

diesel::table! {
    websites (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(logins -> accounts (account));
diesel::joinable!(passwords -> accounts (account));
diesel::joinable!(passwords -> websites (website));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    logins,
    passwords,
    websites,
);
