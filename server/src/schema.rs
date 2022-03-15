table! {
    tokens (token) {
        token -> Varchar,
        owner -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Varchar,
        created_at -> Timestamptz,
        username -> Varchar,
        password -> Varchar,
        password_salt -> Varchar,
        experience -> Float8,
        level -> Int2,
    }
}

joinable!(tokens -> users (owner));

allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
