table! {
    users (id) {
        id -> Integer,
        username -> Text,
        firstname -> Nullable<Text>,
        lastname -> Nullable<Text>,
        password -> Text,
        email -> Nullable<Text>,
    }
}
