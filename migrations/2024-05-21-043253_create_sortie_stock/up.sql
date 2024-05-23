create table sortie_stock(
    id_sortie_stock UUID PRIMARY KEY default gen_random_uuid(),
    date_sortie TIMESTAMP NOT NULL default now(),
    point_vente UUID NOT NULL References point_vente(id_point_vente),
    entree_stock UUID NOT NULL UNIQUE References entree_stock(id_entree_stock)
);
