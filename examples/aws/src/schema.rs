table! {
    bank_accounts (id) {
        id -> Varchar,
        name -> Varchar,
        is_closed -> Bool,
        balance -> Int4,
        created_at -> Timestamptz,
        updated_at: Timestamptz,
        version -> Unsigned<Bigint>,
    }
}
