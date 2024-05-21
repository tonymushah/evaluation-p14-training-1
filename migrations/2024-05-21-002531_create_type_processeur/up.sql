-- Your SQL goes here
create table type_processeur(
    id_type_processeur UUID PRIMARY KEY default gen_random_uuid(),
    puissance DECIMAL NOT NULL,
    designation TEXT NOT NULL,
    marque UUID NOT NULL REFERENCES marque(id_marque)
);
