// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "product_unit"))]
    pub struct ProductUnit;
}

diesel::table! {
    group_messages (id) {
        id -> Uuid,
        message -> Text,
        sender -> Uuid,
        to_group -> Uuid,
        sequence -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    groups (id) {
        id -> Uuid,
        name -> Text,
        created_by_user -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProductUnit;

    items (id) {
        id -> Uuid,
        product_id -> Uuid,
        group_id -> Uuid,
        unit -> Nullable<ProductUnit>,
        quantity -> Nullable<Numeric>,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        message -> Text,
        sender -> Uuid,
        receiver -> Uuid,
        sequence -> Int4,
        read -> Nullable<Bool>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    products (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        nickname -> Text,
        name -> Text,
        surname -> Text,
        email -> Text,
        password -> Text,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    users_groups (id) {
        id -> Uuid,
        group_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::joinable!(group_messages -> groups (to_group));
diesel::joinable!(group_messages -> users (sender));
diesel::joinable!(groups -> users (created_by_user));
diesel::joinable!(items -> groups (group_id));
diesel::joinable!(items -> products (product_id));
diesel::joinable!(users_groups -> groups (group_id));
diesel::joinable!(users_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_messages,
    groups,
    items,
    messages,
    products,
    users,
    users_groups,
);
