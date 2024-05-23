use async_graphql::SimpleObject;
use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::views::v_transfert_en_cours;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_transfert_en_cours)]
#[diesel(primary_key(id_demande, entree_magasin, laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TransfertPendingEntry {
    pub id_demande: Uuid,
    pub date_demande: PrimitiveDateTime,
    pub point_vente: Uuid,
    pub entree_magasin: Uuid,
    pub date_entree_magasin: PrimitiveDateTime,
    pub laptop: Uuid,
}
