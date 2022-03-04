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
