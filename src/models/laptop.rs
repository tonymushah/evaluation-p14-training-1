use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::views::v_laptop;

#[derive(
    Debug,
    Clone,
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
#[diesel(table_name = v_laptop)]
#[diesel(primary_key(id_laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Laptop {
    pub id_laptop: Uuid,
    pub type_processeur: Uuid,
    pub carte_graphique: Uuid,
    pub ecran: BigDecimal,
    pub type_clavier: Uuid,
    pub prix_unitaire: BigDecimal,
    pub ref_laptop: Uuid,
}
