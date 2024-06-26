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

diesel::table! {
    v_transfert_en_cours(id_demande, entree_magasin, laptop) {
        id_demande -> Uuid,
        date_demande -> Timestamp,
        point_vente -> Uuid,
        entree_magasin -> Uuid,
        date_entree_magasin -> Timestamp,
        laptop -> Uuid
    }
}

diesel::table! {
    v_point_vente_sortie_stock(id_sortie_stock, id_entree_stock, id_demande, entree_magasin, laptop) {
        id_sortie_stock -> Uuid,
        date_sortie -> Timestamp,
        vente -> Bool,
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

diesel::table! {
    v_ventes(sortie_stock, entree_stock, id_demande, entree_magasin, laptop) {
        sortie_stock -> Uuid,
        date_sortie -> Timestamp,
        entree_stock -> Uuid,
        entree_date -> Timestamp,
        id_demande -> Uuid,
        date_demande -> Timestamp,
        point_vente -> Uuid,
        entree_magasin -> Uuid,
        date_entree_magasin -> Timestamp,
        laptop -> Uuid,
        prix -> Numeric
    }
}

diesel::table! {
    v_point_vente_stock(id_entree_stock, id_demande, entree_magasin, laptop) {
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

diesel::table! {
    v_point_vente_chiffres(point_vente) {
        point_vente -> Uuid,
        sommes -> Numeric
    }
}

diesel::table! {
    v_point_vente_perdus_somme(point_vente) {
        point_vente -> Uuid,
        sommes -> Numeric
    }
}
