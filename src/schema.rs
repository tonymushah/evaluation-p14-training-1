// @generated automatically by Diesel CLI.

diesel::table! {
    laptop (id_laptop) {
        id_laptop -> Uuid,
        type_processeur -> Nullable<Uuid>,
        carte_graphique -> Nullable<Uuid>,
        ecran -> Nullable<Numeric>,
        type_clavier -> Nullable<Uuid>,
        prix_unitaire -> Nullable<Numeric>,
        ref_laptop -> Uuid,
    }
}

diesel::table! {
    marque (id_marque) {
        id_marque -> Uuid,
        desination -> Text,
    }
}

diesel::table! {
    point_vente (id_point_vente) {
        id_point_vente -> Uuid,
        designation -> Text,
    }
}

diesel::table! {
    ram_laptop (id_ram_laptop) {
        id_ram_laptop -> Uuid,
        marque -> Uuid,
        puissance -> Numeric,
        type_ram -> Uuid,
        frequence -> Numeric,
        laptop -> Uuid,
    }
}

diesel::table! {
    ram_ref_laptop (id_ram_ref) {
        id_ram_ref -> Uuid,
        marque -> Uuid,
        puissance -> Numeric,
        type_ram -> Uuid,
        frequence -> Numeric,
        ref_laptop -> Uuid,
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

diesel::joinable!(laptop -> reference_laptop (ref_laptop));
diesel::joinable!(laptop -> type_carte_graphique (carte_graphique));
diesel::joinable!(laptop -> type_clavier (type_clavier));
diesel::joinable!(laptop -> type_processeur (type_processeur));
diesel::joinable!(ram_laptop -> laptop (laptop));
diesel::joinable!(ram_laptop -> marque (marque));
diesel::joinable!(ram_laptop -> type_ram (type_ram));
diesel::joinable!(ram_ref_laptop -> marque (marque));
diesel::joinable!(ram_ref_laptop -> reference_laptop (ref_laptop));
diesel::joinable!(ram_ref_laptop -> type_ram (type_ram));
diesel::joinable!(reference_laptop -> type_carte_graphique (carte_graphique));
diesel::joinable!(reference_laptop -> type_clavier (type_clavier));
diesel::joinable!(reference_laptop -> type_processeur (type_processeur));
diesel::joinable!(type_carte_graphique -> marque (marque));
diesel::joinable!(type_processeur -> marque (marque));

diesel::allow_tables_to_appear_in_same_query!(
    laptop,
    marque,
    point_vente,
    ram_laptop,
    ram_ref_laptop,
    reference_laptop,
    type_carte_graphique,
    type_clavier,
    type_processeur,
    type_ram,
);
