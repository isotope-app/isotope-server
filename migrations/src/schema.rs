table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Text,
        bio -> Nullable<Varchar>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        last_online -> Timestamp,
    }
}
