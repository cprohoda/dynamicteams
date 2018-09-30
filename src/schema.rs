table! {
    users (id) {
        id -> Int4,
        email -> Text,
        skills -> Nullable<Array<Text>>,
        tasks -> Nullable<Array<Text>>,
    }
}
