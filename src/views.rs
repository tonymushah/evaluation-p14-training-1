diesel::table! {
    v_ram_ref_laptop (ref_laptop) {
        ref_laptop -> Uuid,
        puissance -> Decimal,
        frequence -> Decimal
    }
}

diesel::table! {
    v_ram_laptop (laptop) {
        laptop -> Uuid,
        puissance -> Decimal,
        frequence -> Decimal
    }
}

diesel::table! {
    v_laptop (id_laptop) {
        id_laptop -> Uuid,
        nom -> Text,
        type_processeur -> Uuid,
        carte_graphique -> Uuid,
        ecran -> Numeric,
        type_clavier -> Uuid,
        prix_unitaire -> Numeric,
        ref_laptop -> Uuid
    }
}

diesel::table! {
    v_magasin_stock (id_entree_stock) {
        id_entree_stock -> Uuid,
        laptop -> Uuid,
        entree_date -> Timestamp,
    }
}
