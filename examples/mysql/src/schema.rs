table! {
    event_stores (event_id) {
        event_id -> Unsigned<Bigint>,
        event_body -> Text,
        event_type -> Varchar,
        stream_id -> Varchar,
        stream_version -> Unsigned<Bigint>,
        event_occurred_at -> Datetime,
    }
}

table! {
    snapshots (stream_id) {
        stream_id -> Varchar,
        stream_version -> Unsigned<Bigint>,
        data -> Text,
        created_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    event_stores,
    snapshots,
);
