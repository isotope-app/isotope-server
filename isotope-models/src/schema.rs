table! {
    instances (id) {
        id -> Integer,
        local_domain -> Text,
        public_domain -> Text,
        name -> Text,
        local -> Bool,
        blocked -> Bool,
    }
}

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
        instance_id -> Integer,
    }
}

joinable!(users -> instances (instance_id));

allow_tables_to_appear_in_same_query!(
    instances,
    users,
);
