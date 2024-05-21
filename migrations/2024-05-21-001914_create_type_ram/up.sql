-- Your SQL goes here
create table type_ram(
    id_type UUID PRIMARY KEY default gen_random_uuid(),
    designation TEXT NOT NULL 
);
