use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::schema::entree_stock;

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
    AsChangeset,
)]
#[diesel(table_name = entree_stock)]
#[diesel(primary_key(id_entree_stock))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntreeStock {
    pub id_entree_stock: Uuid,
    pub laptop: Uuid,
    pub entree_date: PrimitiveDateTime,
}
