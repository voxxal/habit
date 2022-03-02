table! {
    users (id) {
        id -> Varchar,
        created_at -> Timestamptz,
        experience -> Nullable<Float8>,
        level -> Nullable<Int2>,
    }
}
