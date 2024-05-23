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

diesel::table! {
    v_demande_transfert(id_demande, entree_magasin, laptop) {
        id_demande -> Uuid,
        date_demande -> Timestamp,
        point_vente -> Uuid,
        entree_magasin -> Uuid,
        date_entree_magasin -> Timestamp,
        laptop -> Uuid
    }
}

diesel::table! {
    v_entree_stock_point_vente(id_entree_stock, id_demande, entree_magasin, laptop) {
        id_entree_stock -> Uuid,
        entree_date -> Timestamp,
        id_demande -> Uuid,
        date_demande -> Timestamp,
        point_vente -> Uuid,
        entree_magasin -> Uuid,
        date_entree_magasin -> Timestamp,
        laptop -> Uuid
    }
}
