use async_graphql::{InputObject, SimpleObject};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::type_processeur;

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
#[graphql(input_name = "TypeProcesseurInput")]
#[diesel(table_name = type_processeur)]
#[diesel(primary_key(id_type_processeur))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TypeProcesseur {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_type_processeur: Uuid,
    pub designation: String,
    pub marque: Uuid,
    pub puissance: BigDecimal,
}
