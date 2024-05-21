-- Your SQL goes here
create table point_vente(
    id_point_vente UUID PRIMARY KEY default gen_random_uuid(),
    designation TEXT NOT NULL
);
