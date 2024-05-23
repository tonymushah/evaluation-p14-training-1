-- Your SQL goes here
CREATE VIEW V_VENTES AS
SELECT v_pvss.id_sortie_stock AS sortie_stock,
    v_pvss.date_sortie,
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