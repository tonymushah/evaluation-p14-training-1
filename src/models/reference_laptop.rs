use async_graphql::{InputObject, SimpleObject};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::reference_laptop;

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
    SimpleObject,
    InputObject,
)]
#[graphql(input_name = "ReferenceLaptopInput")]
#[diesel(table_name = reference_laptop)]
#[diesel(primary_key(id_ref))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReferenceLaptop {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_ref: Uuid,
    pub type_processeur: Uuid,
    pub carte_graphique: Uuid,
    pub ecran: BigDecimal,
    pub type_clavier: Uuid,
    pub prix_unitaire: BigDecimal,
}
