// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "product_unit"))]
    pub struct ProductUnit;
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

diesel::joinable!(groups -> users (created_by_user));
diesel::joinable!(items -> groups (group_id));
diesel::joinable!(items -> products (product_id));
diesel::joinable!(users_groups -> groups (group_id));
diesel::joinable!(users_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    items,
    products,
    users,
    users_groups,
);
