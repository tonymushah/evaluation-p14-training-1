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