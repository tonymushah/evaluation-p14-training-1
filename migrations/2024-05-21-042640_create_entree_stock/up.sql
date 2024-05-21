-- Your SQL goes here
create table entree_stock (
    id_entree_stock UUID PRIMARY KEY default gen_random_uuid(),
    laptop UUID NoT NULL References laptop(id_laptop),
    entree_date TIMESTAMP NOT NULL default now()
);
