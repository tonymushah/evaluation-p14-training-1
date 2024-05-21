-- Your SQL goes here
create table reference_laptop(
    id_ref UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nom TEXT NOT NULL,
    type_processeur UUID NOT NULL REFERENCES type_processeur(id_type_processeur),
    carte_graphique UUID NOT NULL REFERENCES type_carte_graphique(id_type),
    ecran DECIMAL NOT NULL,
    type_clavier UUID NOT NULL REFERENCES type_clavier(id_type_clavier),
    prix_unitaire DECIMAL NOT NULL
);
