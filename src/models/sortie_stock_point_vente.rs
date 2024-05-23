use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::schema::sortie_stock_point_vente;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Identifiable,
    Selectable,
    Insertable,
    Queryable,
)]
#[diesel(table_name = sortie_stock_point_vente)]
#[diesel(primary_key(id_sortie_stock))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SortieStockPointVente {
    pub id_sortie_stock: Uuid,
    pub date_sortie: PrimitiveDateTime,
    pub entree_stock: Uuid,
    pub vente: bool,
}
