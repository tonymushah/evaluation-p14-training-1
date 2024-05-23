use async_graphql::SimpleObject;
use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::views::v_point_vente_stock;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_point_vente_stock)]
#[diesel(primary_key(id_entree_stock, id_demande, entree_magasin, laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PointVenteStockEntry {
    pub id_entree_stock: Uuid,
    pub entree_date: PrimitiveDateTime,
    pub id_demande: Uuid,
    pub date_demande: PrimitiveDateTime,
    pub point_vente: Uuid,
    pub entree_magasin: Uuid,
    pub date_entree_magasin: PrimitiveDateTime,
    pub laptop: Uuid,
}
