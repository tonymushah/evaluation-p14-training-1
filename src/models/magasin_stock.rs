use async_graphql::{ComplexObject, Context, SimpleObject};
use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{graphql::GetPoolConnection, views::v_magasin_stock};

use super::laptop::Laptop;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[graphql(complex)]
#[diesel(table_name = v_magasin_stock)]
#[diesel(primary_key(id_entree_stock))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MagasinStockEntry {
    pub id_entree_stock: Uuid,
    #[graphql(skip)]
    pub laptop: Uuid,
    pub entree_date: PrimitiveDateTime,
}

#[ComplexObject]
impl MagasinStockEntry {
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
