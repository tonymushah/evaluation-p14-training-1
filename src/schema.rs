// @generated automatically by Diesel CLI.

diesel::table! {
    marque (id_marque) {
        id_marque -> Uuid,
        desination -> Text,
    }
}

diesel::table! {
    reference_laptop (id_ref) {
        id_ref -> Uuid,
        nom -> Text,
        type_processeur -> Uuid,
        carte_graphique -> Uuid,
        ecran -> Numeric,
        type_clavier -> Uuid,
        prix_unitaire -> Numeric,
    }
}

diesel::table! {
    type_carte_graphique (id_type) {
        id_type -> Uuid,
        marque -> Uuid,
        designation -> Text,
        puissance -> Numeric,
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

diesel::joinable!(reference_laptop -> type_carte_graphique (carte_graphique));
diesel::joinable!(reference_laptop -> type_clavier (type_clavier));
diesel::joinable!(reference_laptop -> type_processeur (type_processeur));
diesel::joinable!(type_carte_graphique -> marque (marque));
diesel::joinable!(type_processeur -> marque (marque));

diesel::allow_tables_to_appear_in_same_query!(
    marque,
    reference_laptop,
    type_carte_graphique,
    type_clavier,
    type_processeur,
    type_ram,
);
