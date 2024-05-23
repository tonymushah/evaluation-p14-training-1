use async_graphql::SimpleObject;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::views::v_ventes;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_ventes)]
#[diesel(primary_key(sortie_stock, entree_stock, id_demande, entree_magasin, laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vente {
    pub sortie_stock: Uuid,
    pub date_sortie: PrimitiveDateTime,
    pub entree_stock: Uuid,
    pub entree_date: PrimitiveDateTime,
    pub id_demande: Uuid,
    pub date_demande: PrimitiveDateTime,
    pub point_vente: Uuid,
    pub entree_magasin: Uuid,
    pub date_entree_magasin: PrimitiveDateTime,
    pub laptop: Uuid,
    pub prix: BigDecimal,
}
