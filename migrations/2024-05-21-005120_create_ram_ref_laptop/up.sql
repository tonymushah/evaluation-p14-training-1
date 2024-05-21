-- Your SQL goes here
create table ram_ref_laptop(
    id_ram_ref UUID PRIMARY KEY default gen_random_uuid(),
    marque UUID NOT NULL REFERENCES marque(id_marque),
    puissance DECIMAL NOT NULL,
    type_ram UUID NOT NULL REFERENCES type_ram(id_type),
    frequence DECIMAL NOT NULL,
    ref_laptop UUID NOT NULL REFERENCES reference_laptop(id_ref)
);
