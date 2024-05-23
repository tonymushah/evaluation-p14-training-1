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

-- Your SQL goes here
CREATE VIEW V_LAPTOP as 
SELECT ID_LAPTOP,
	COALESCE(LAPTOP.TYPE_PROCESSEUR,

		REF_LAP.TYPE_PROCESSEUR) AS TYPE_PROCESSEUR,
	COALESCE(LAPTOP.CARTE_GRAPHIQUE,

		REF_LAP.CARTE_GRAPHIQUE) AS CARTE_GRAPHIQUE,
	COALESCE(LAPTOP.ECRAN,

		REF_LAP.ECRAN) AS ECRAN,
	COALESCE(LAPTOP.TYPE_CLAVIER,

		REF_LAP.TYPE_CLAVIER) AS TYPE_CLAVIER,
	COALESCE(LAPTOP.PRIX_UNITAIRE,

		REF_LAP.PRIX_UNITAIRE) AS PRIX_UNITAIRE,
	LAPTOP.REF_LAPTOP AS REF_LAPTOP
FROM LAPTOP
JOIN REFERENCE_LAPTOP AS REF_LAP ON LAPTOP.REF_LAPTOP = REF_LAP.ID_REF;

CREATE FUNCTION V_LAPTOP_TRIGGER_FUNC() RETURNS TRIGGER LANGUAGE PLPGSQL AS $$
BEGIN
	If TG_OP = 'INSERT' THEN
		Insert INTO
			laptop(
				id_laptop,
				type_processeur,
				carte_graphique,
				ecran,
				type_clavier,
				prix_unitaire,
				ref_laptop
			)
		values (
			NEW.id_laptop,
			NEW.type_processeur,
			NEW.carte_graphique,
			NEW.ecran,
			NEW.type_clavier,
			NEW.prix_unitaire,
			NEW.ref_laptop
		);
		return NEW;
	ELSEIF TG_OP = 'UPDATE' THEN
		UPDATE laptop SET
			type_processeur = NEW.type_processeur,
			carte_graphique = NEW.carte_graphique,
			ecran = NEW.ecran,
			type_clavier = NEW.type_clavier,
			prix_unitaire = NEW.prix_unitaire,
			ref_laptop = NEW.ref_laptop
		WHERE id_laptop = NEW.id_laptop;
		return NEW;
	ELSEIF TG_OP = 'DELETE' then
		DELETE FROM laptop
		where id_laptop = OLD.id_laptop;
	END IF;
	RETURN NULL;
END;
$$;


CREATE TRIGGER V_LAPTOP_TRIGGER INSTEAD OF
INSERT
OR
UPDATE
OR
DELETE ON V_LAPTOP
FOR EACH ROW EXECUTE FUNCTION V_LAPTOP_TRIGGER_FUNC();

create table point_vente(
    id_point_vente UUID PRIMARY KEY default gen_random_uuid(),
    designation TEXT NOT NULL
);

create table entree_stock (
    id_entree_stock UUID PRIMARY KEY default gen_random_uuid(),
    laptop UUID NoT NULL References laptop(id_laptop),
    entree_date TIMESTAMP NOT NULL default now()
);

create table sortie_stock(
    id_sortie_stock UUID PRIMARY KEY default gen_random_uuid(),
    date_sortie TIMESTAMP NOT NULL default now(),
    point_vente UUID NOT NULL References point_vente(id_point_vente),
    entree_stock UUID NOT NULL References entree_stock(id_entree_stock)
);

create table entree_stock_point_vente(
    id_entree_stock UUID PRIMARY KEY default gen_random_uuid(),
    entree_date TIMESTAMP default now(),
    sortie_stock UUID NOT NULL References sortie_stock(id_sortie_stock)
);

create table sortie_stock_point_vente(
    id_sortie_stock UUID PRIMARY KEY default gen_random_uuid(),
    entree_stock UUID NOT NULL References entree_stock_point_vente(id_entree_stock),
    date_sortie TIMESTAMP NOT NULL default now(),
    vente boolean NOT NULL
);

CREATE VIEW V_MAGASIN_STOCK AS
SELECT E.ID_ENTREE_STOCK,
	E.LAPTOP,
	E.ENTREE_DATE
FROM SORTIE_STOCK AS S
RIGHT JOIN ENTREE_STOCK AS E ON S.ENTREE_STOCK = E.ID_ENTREE_STOCK
WHERE S.ID_SORTIE_STOCK IS NULL;

CREATE VIEW V_DEMANDE_TRANSFERT AS
SELECT S.ID_SORTIE_STOCK AS ID_DEMANDE,
	S.DATE_SORTIE AS DATE_DEMANDE,
	S.POINT_VENTE AS POINT_VENTE,
	E.ID_ENTREE_STOCK AS ENTREE_MAGASIN,
	E.ENTREE_DATE AS DATE_ENTREE_MANGASIN,
	E.LAPTOP AS LAPTOP
FROM SORTIE_STOCK AS S
JOIN ENTREE_STOCK AS E ON S.ENTREE_STOCK = E.ID_ENTREE_STOCK;

CREATE VIEW V_ENTREE_STOCK_POINT_VENTE AS
SELECT EPV.ID_ENTREE_STOCK,
	EPV.ENTREE_DATE,
	V_DT.ID_DEMANDE,
	V_DT.DATE_DEMANDE,
	V_DT.POINT_VENTE,
	V_DT.ENTREE_MAGASIN,
	V_DT.DATE_ENTREE_MAGASIN,
	V_DT.LAPTOP
FROM ENTREE_STOCK_POINT_VENTE AS EPV
JOIN V_DEMANDE_TRANSFERT AS V_DT ON EPV.SORTIE_STOCK = V_DT.ID_DEMANDE;

CREATE VIEW V_TRANSFERT_EN_COURS AS
SELECT V_DT.ID_DEMANDE,
	V_DT.DATE_DEMANDE,
	V_DT.POINT_VENTE,
	V_DT.ENTREE_MAGASIN,
	V_DT.DATE_ENTREE_MAGASIN,
	V_DT.LAPTOP
FROM ENTREE_STOCK_POINT_VENTE AS EPV
RIGHT JOIN V_DEMANDE_TRANSFERT AS V_DT ON EPV.SORTIE_STOCK = V_DT.ID_DEMANDE
WHERE EPV.ID_ENTREE_STOCK IS NULL ;

CREATE VIEW V_POINT_VENTE_SORTIE_STOCK AS
SELECT SSPV.ID_SORTIE_STOCK,
	SSPV.DATE_SORTIE,
	SSPV.VENTE,
	V_ESPV.ID_ENTREE_STOCK,
	V_ESPV.ENTREE_DATE,
	V_ESPV.ID_DEMANDE,
	V_ESPV.DATE_DEMANDE,
	V_ESPV.POINT_VENTE,
	V_ESPV.ENTREE_MAGASIN,
	V_ESPV.DATE_ENTREE_MAGASIN,
	V_ESPV.LAPTOP
FROM SORTIE_STOCK_POINT_VENTE AS SSPV
JOIN V_ENTREE_STOCK_POINT_VENTE AS V_ESPV ON SSPV.ENTREE_STOCK = V_ESPV.ID_ENTREE_STOCK;

CREATE VIEW V_VENTES AS
SELECT v_pvss.id_sortie_stock AS sortie_stock,
    v_pvss.date_sortie,
    v_pvss.vente,
    v_pvss.id_entree_stock AS entree_stock,
    v_pvss.entree_date,
    v_pvss.id_demande,
    v_pvss.date_demande,
    v_pvss.point_vente,
    v_pvss.entree_magasin,
    v_pvss.date_entree_magasin,
    v_pvss.laptop,
    v_laptop.prix_unitaire AS prix
   FROM v_point_vente_sortie_stock v_pvss
     JOIN v_laptop ON v_laptop.id_laptop = v_pvss.laptop
  WHERE v_pvss.vente = true;

CREATE VIEW V_POINT_VENTE_STOCK AS
SELECT V_ESPV.ID_ENTREE_STOCK,
	V_ESPV.ENTREE_DATE,
	V_ESPV.ID_DEMANDE,
	V_ESPV.DATE_DEMANDE,
	V_ESPV.POINT_VENTE,
	V_ESPV.ENTREE_MAGASIN,
	V_ESPV.DATE_ENTREE_MAGASIN,
	V_ESPV.LAPTOP
FROM SORTIE_STOCK_POINT_VENTE AS SSPV
JOIN V_ENTREE_STOCK_POINT_VENTE AS V_ESPV ON SSPV.ENTREE_STOCK = V_ESPV.ID_ENTREE_STOCK
WHERE SSPV.ID_SORTIE_STOCK IS NULL;

CREATE VIEW V_POINT_VENTE_CHIFFRES AS
SELECT POINT_VENTE,
	SUM(PRIX) AS SOMMES
FROM V_VENTES
GROUP BY POINT_VENTE;

CREATE VIEW V_POINT_VENTE_PERDUS_SOMME AS
SELECT V_TEC.POINT_VENTE,
	SUM(V_L.PRIX_UNITAIRE) AS SOMMES
FROM V_TRANSFERT_EN_COURS AS V_TEC
JOIN V_LAPTOP AS V_L ON V_TEC.LAPTOP = V_L.ID_LAPTOP
GROUP BY V_TEC.POINT_VENTE;
