use async_graphql::{InputObject, SimpleObject};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::type_carte_graphique as type_graphique;

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
#[graphql(input_name = "TypeCarteGraphiqueInput")]
#[diesel(table_name = type_graphique)]
#[diesel(primary_key(id_type))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TypeCarteGraphique {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_type: Uuid,
    pub designation: String,
    pub marque: Uuid,
    pub puissance: BigDecimal,
}
