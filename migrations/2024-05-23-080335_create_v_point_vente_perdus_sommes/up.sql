-- Your SQL goes here
CREATE VIEW V_POINT_VENTE_PERDUS_SOMME AS
SELECT V_TEC.POINT_VENTE,
	SUM(V_L.PRIX_UNITAIRE) AS SOMMES
FROM V_TRANSFERT_EN_COURS AS V_TEC
JOIN V_LAPTOP AS V_L ON V_TEC.LAPTOP = V_L.ID_LAPTOP
GROUP BY V_TEC.POINT_VENTE;