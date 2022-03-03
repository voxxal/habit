table! {
    users (id) {
        id -> Varchar,
        created_at -> Timestamptz,
	username -> String,
	password -> String,
	salt -> String,
        experience -> Nullable<Float8>,
        level -> Nullable<Int2>,
    }
}
