// @generated automatically by Diesel CLI.

diesel::table! {
    marque (id_marque) {
        id_marque -> Uuid,
        desination -> Text,
    }
}

diesel::table! {
    type_clavier (id_type_clavier) {
        id_type_clavier -> Uuid,
        designation -> Text,
    }
}

diesel::table! {
    type_processeur (id_type_processeur) {
        id_type_processeur -> Uuid,
        puissance -> Numeric,
        designation -> Text,
        marque -> Uuid,
    }
}

diesel::table! {
    type_ram (id_type) {
        id_type -> Uuid,
        designation -> Text,
    }
}

diesel::joinable!(type_processeur -> marque (marque));

diesel::allow_tables_to_appear_in_same_query!(
    marque,
    type_clavier,
    type_processeur,
    type_ram,
);
