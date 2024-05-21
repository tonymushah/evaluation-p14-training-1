// @generated automatically by Diesel CLI.

diesel::table! {
    marque (id_marque) {
        id_marque -> Uuid,
        desination -> Text,
    }
}

diesel::table! {
    type_ram (id_type) {
        id_type -> Uuid,
        designation -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    marque,
    type_ram,
);
