use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::schema::entree_stock_point_vente;

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
#[diesel(table_name = entree_stock_point_vente)]
#[diesel(primary_key(id_entree_stock))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntreeStockPointVente {
    pub id_entree_stock: Uuid,
    pub entree_date: Option<PrimitiveDateTime>,
    pub sortie_stock: Uuid,
}
