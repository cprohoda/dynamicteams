table! {
    orgs (id) {
        id -> Int4,
        code -> Text,
        name -> Nullable<Text>,
        employees -> Nullable<Array<Int4>>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        skills -> Nullable<Array<Text>>,
        tasks -> Nullable<Array<Text>>,
    }
}

allow_tables_to_appear_in_same_query!(
    orgs,
    users,
);
