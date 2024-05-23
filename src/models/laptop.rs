use async_graphql::InputObject;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{schema::laptop, views::v_laptop};

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
    InputObject,
)]
#[diesel(table_name = laptop)]
#[diesel(primary_key(id_laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LaptopInput {
    pub id_laptop: Uuid,
    pub type_processeur: Option<Uuid>,
    pub carte_graphique: Option<Uuid>,
    pub ecran: Option<BigDecimal>,
    pub type_clavier: Option<Uuid>,
    pub prix_unitaire: Option<BigDecimal>,
    pub ref_laptop: Uuid,
}
