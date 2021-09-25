table! {
    user (email) {
        email -> Varchar,
        username -> Varchar,
        hashed_password -> Bpchar,
        registration_date -> Date,
        activation_date -> Nullable<Date>,
        locked -> Bool,
    }
}
