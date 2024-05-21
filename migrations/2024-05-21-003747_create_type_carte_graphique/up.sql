-- Your SQL goes here
create table type_carte_graphique(
    id_type UUID PRIMARY KEY default gen_random_uuid(),
    marque UUID NOT NULL REFERENCES marque(id_marque),
    designation TEXT NOT NULL,
    puissance DECIMAL NOT NULL
);
