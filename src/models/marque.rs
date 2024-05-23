use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::marque;

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
#[graphql(input_name = "MarqueInput")]
#[diesel(table_name = marque)]
#[diesel(primary_key(id_marque))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Marque {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_marque: Uuid,
    pub desination: String,
}
