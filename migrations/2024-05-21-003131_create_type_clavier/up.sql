-- Your SQL goes here
create table type_clavier(
    id_type_clavier UUID PRIMARY KEY default gen_random_uuid(),
    designation TEXT NOT NULL
);
