use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::type_clavier;

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
#[graphql(input_name = "TypeClavierInput")]
#[diesel(table_name = type_clavier)]
#[diesel(primary_key(id_type_clavier))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TypeClavier {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_type_clavier: Uuid,
    pub designation: String,
}
