table! {
    tiles (id) {
        id -> Varchar,
        owner -> Varchar,
        name -> Varchar,
        completion -> Nullable<Bytea>,
        #[sql_name = "type"]
        type_ -> Int2,
    }
}

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

joinable!(tiles -> users (owner));
joinable!(tokens -> users (owner));

allow_tables_to_appear_in_same_query!(tiles, tokens, users,);
