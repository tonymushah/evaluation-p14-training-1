-- Your SQL goes here
create table entree_stock_point_vente(
    id_entree_stock UUID PRIMARY KEY default gen_random_uuid(),
    entree_date TIMESTAMP default now(),
    sortie_stock UUID NOT NULL UNIQUE References sortie_stock(id_sortie_stock)
);
