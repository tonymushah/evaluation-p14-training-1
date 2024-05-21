-- Your SQL goes here
create table laptop(
    id_laptop UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_processeur UUID REFERENCES type_processeur(id_type_processeur),
    carte_graphique UUID REFERENCES type_carte_graphique(id_type),
    ecran DECIMAL,
    type_clavier UUID REFERENCES type_clavier(id_type_clavier),
    prix_unitaire DECIMAL,
    ref_laptop UUID NOT NULL REFERENCES reference_laptop(id_ref)
);
