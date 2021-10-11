table! {
    users (email) {
        email -> Varchar,
        username -> Varchar,
        hashed_password -> Bpchar,
        registration_date -> Timestamptz,
        activation_date -> Nullable<Timestamptz>,
        locked -> Bool,
    }
}
