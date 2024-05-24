use async_graphql::{ComplexObject, Context, SimpleObject};
use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{graphql::GetPoolConnection, views::v_demande_transfert};

use super::{laptop::Laptop, point_vente::PointVente};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_demande_transfert)]
#[diesel(primary_key(id_demande, entree_magasin, laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[graphql(complex)]
pub struct TransfertDemandEntry {
    pub id_demande: Uuid,
    pub date_demande: PrimitiveDateTime,
    #[graphql(skip)]
    pub point_vente: Uuid,
    pub entree_magasin: Uuid,
    pub date_entree_magasin: PrimitiveDateTime,
    #[graphql(skip)]
    pub laptop: Uuid,
}

#[ComplexObject]
impl TransfertDemandEntry {
    pub async fn point_vente(&self, ctx: &Context<'_>) -> crate::Result<PointVente> {
        let point_vente_id = self.point_vente;
        ctx.use_pool(move |mut pool| {
            use crate::schema::point_vente::dsl::*;
            Ok(point_vente
                .filter(id_point_vente.eq(point_vente_id))
                .select(PointVente::as_select())
                .get_result(&mut pool)?)
        })
        .await
    }
    pub async fn laptop(&self, ctx: &Context<'_>) -> crate::Result<Laptop> {
        let laptop = self.laptop;
        ctx.use_pool(move |mut pool| {
            use crate::views::v_laptop::dsl::*;
            Ok(v_laptop
                .filter(id_laptop.eq(laptop))
                .select(Laptop::as_select())
                .get_result(&mut pool)?)
        })
        .await
    }
}
