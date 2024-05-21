-- Your SQL goes here
create table sortie_stock_point_vente(
    id_sortie_stock UUID PRIMARY KEY default gen_random_uuid(),
    entree_stock UUID NOT NULL References entree_stock_point_vente(id_entree_stock),
    date_sortie TIMESTAMP NOT NULL default now(),
    vente boolean NOT NULL
);

