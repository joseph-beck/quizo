diesel::table! {
    users (uuid) {
        uuid -> VarChar,
        username -> VarChar,
        password -> VarChar,
        email -> VarChar,
    }
}
