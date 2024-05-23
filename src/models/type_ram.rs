use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::type_ram;

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
#[graphql(input_name = "TypeRamInput")]
#[diesel(table_name = type_ram)]
#[diesel(primary_key(id_type))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TypeRam {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_type: Uuid,
    pub designation: String,
}
