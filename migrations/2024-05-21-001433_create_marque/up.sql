-- Your SQL goes here
create table marque(
    id_marque UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    desination TEXT NOT NULL 
);
