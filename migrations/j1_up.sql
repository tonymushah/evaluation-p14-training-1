create table marque(
    id_marque UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    desination TEXT NOT NULL 
);

create table type_ram(
    id_type UUID PRIMARY KEY default gen_random_uuid(),
    designation TEXT NOT NULL 
);

create table type_processeur(
    id_type_processeur UUID PRIMARY KEY default gen_random_uuid(),
    puissance DECIMAL NOT NULL,
    designation TEXT NOT NULL,
    marque UUID NOT NULL REFERENCES marque(id_marque)
);

create table type_clavier(
    id_type_clavier UUID PRIMARY KEY default gen_random_uuid(),
    designation TEXT NOT NULL
);

create table type_carte_graphique(
    id_type UUID PRIMARY KEY default gen_random_uuid(),
    marque UUID NOT NULL REFERENCES marque(id_marque),
    designation TEXT NOT NULL,
    puissance DECIMAL NOT NULL
);

create table reference_laptop(
    id_ref UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nom TEXT NOT NULL,
    type_processeur UUID NOT NULL REFERENCES type_processeur(id_type_processeur),
    carte_graphique UUID NOT NULL REFERENCES type_carte_graphique(id_type),
    ecran DECIMAL NOT NULL,
    type_clavier UUID NOT NULL REFERENCES type_clavier(id_type_clavier),
    prix_unitaire DECIMAL NOT NULL
);

create table ram_ref_laptop(
    id_ram_ref UUID PRIMARY KEY default gen_random_uuid(),
    marque UUID NOT NULL REFERENCES marque(id_marque),
    puissance DECIMAL NOT NULL,
    type_ram UUID NOT NULL REFERENCES type_ram(id_type),
    frequence DECIMAL NOT NULL,
    ref_laptop UUID NOT NULL REFERENCES reference_laptop(id_ref)
);

CREATE VIEW V_RAM_REF_LAPTOP AS
SELECT REF_LAPTOP,
	SUM(PUISSANCE) AS PUISSANCE,
	AVG(FREQUENCE) AS FREQUENCE
FROM RAM_REF_LAPTOP
GROUP BY REF_LAPTOP;

create table laptop(
    id_laptop UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_processeur UUID REFERENCES type_processeur(id_type_processeur),
    carte_graphique UUID REFERENCES type_carte_graphique(id_type),
    ecran DECIMAL,
    type_clavier UUID REFERENCES type_clavier(id_type_clavier),
    prix_unitaire DECIMAL,
    ref_laptop UUID NOT NULL REFERENCES reference_laptop(id_ref)
);

create table ram_laptop(
    id_ram_laptop UUID PRIMARY KEY default gen_random_uuid(),
    marque UUID NOT NULL REFERENCES marque(id_marque),
    puissance DECIMAL NOT NULL,
    type_ram UUID NOT NULL REFERENCES type_ram(id_type),
    frequence DECIMAL NOT NULL,
    laptop UUID NOT NULL REFERENCES laptop(id_laptop)
);

CREATE VIEW V_RAM_LAPTOP AS
SELECT LAPTOP,
	SUM(PUISSANCE) AS PUISSANCE,
	AVG(FREQUENCE) AS FREQUENCE
FROM RAM_LAPTOP
GROUP BY LAPTOP;