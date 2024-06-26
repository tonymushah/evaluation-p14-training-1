-- Your SQL goes here
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