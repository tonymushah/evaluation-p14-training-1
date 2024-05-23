use async_graphql::SimpleObject;
use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::views::v_magasin_stock;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_magasin_stock)]
#[diesel(primary_key(id_entree_stock))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MagasinStockEntry {
    pub id_entree_stock: Uuid,
    pub laptop: Uuid,
    pub entree_date: PrimitiveDateTime,
}
