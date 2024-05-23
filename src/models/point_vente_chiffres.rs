use async_graphql::SimpleObject;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::views::v_point_vente_chiffres;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_point_vente_chiffres)]
#[diesel(primary_key(point_vente))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PointVenteChiffre {
    pub point_vente: Uuid,
    pub sommes: BigDecimal,
}
