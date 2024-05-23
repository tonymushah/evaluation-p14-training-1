-- Your SQL goes here
CREATE VIEW V_MAGASIN_STOCK AS
SELECT E.ID_ENTREE_STOCK,
	E.LAPTOP,
	E.ENTREE_DATE
FROM SORTIE_STOCK AS S
RIGHT JOIN ENTREE_STOCK AS E ON S.ENTREE_STOCK = E.ID_ENTREE_STOCK
WHERE S.ID_SORTIE_STOCK IS NULL;