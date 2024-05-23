use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::point_vente;

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
#[graphql(input_name = "PointVenteInput")]
#[diesel(table_name = point_vente)]
#[diesel(primary_key(id_point_vente))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PointVente {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_point_vente: Uuid,
    pub designation: String,
}
