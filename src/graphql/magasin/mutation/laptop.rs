use actix_web::web::block;
use async_graphql::{Context, Object};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    graphql::GetPoolConnection,
    models::laptop::{Laptop, LaptopInput},
    DbPoolConnection,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct LaptopCrudMutations;

impl LaptopCrudMutations {
    fn get_laptop(laptop_id: Uuid, con: &mut DbPoolConnection) -> crate::Result<Laptop> {
        use crate::views::v_laptop::dsl::*;
        Ok(v_laptop
            .filter(id_laptop.eq(laptop_id))
            .select(Laptop::as_select())
            .get_result(con)?)
    }
    fn get_laptops(laptop_ids: &[Uuid], con: &mut DbPoolConnection) -> crate::Result<Vec<Laptop>> {
        use crate::views::v_laptop::dsl::*;
        Ok(v_laptop
            .filter(id_laptop.eq_any(laptop_ids))
            .select(Laptop::as_select())
            .get_results(con)?)
    }
}

#[Object]
impl LaptopCrudMutations {
    pub async fn upsert(&self, ctx: &Context<'_>, input: LaptopInput) -> crate::Result<Laptop> {
        let mut pool = ctx.pool()?;
        block(move || -> crate::Result<Laptop> {
            use crate::schema::laptop::dsl::*;
            let res: LaptopInput = diesel::insert_into(laptop)
                .values(&input)
                .on_conflict(id_laptop)
                .do_update()
                .set(&input)
                .returning(LaptopInput::as_returning())
                .get_result(&mut pool)?;
            Self::get_laptop(res.id_laptop, &mut pool)
        })
        .await?
    }
    pub async fn upsert_batch(
        &self,
        ctx: &Context<'_>,
        input: Vec<LaptopInput>,
    ) -> crate::Result<Vec<Laptop>> {
        let mut pool = ctx.pool()?;
        block(move || -> crate::Result<Vec<Laptop>> {
            use crate::schema::laptop::dsl::*;
            let mut res: Vec<LaptopInput> = diesel::insert_into(laptop)
                .values(&input)
                .on_conflict(id_laptop)
                .do_nothing()
                .returning(LaptopInput::as_returning())
                .get_results(&mut pool)?;
            for in_ in input.iter() {
                res.push(
                    diesel::update(laptop)
                        .set(in_)
                        .returning(LaptopInput::as_returning())
                        .get_result(&mut pool)?,
                );
            }
            Self::get_laptops(
                &res.into_iter().map(|r| r.id_laptop).collect::<Vec<_>>(),
                &mut pool,
            )
        })
        .await?
    }
    pub async fn delete(&self, ctx: &Context<'_>, id: Uuid) -> crate::Result<Laptop> {
        let mut pool = ctx.pool()?;
        block(move || -> crate::Result<Laptop> {
            use crate::schema::laptop::dsl::*;
            let to_use = Self::get_laptop(id, &mut pool)?;
            diesel::delete(laptop.filter(id_laptop.eq(&to_use.id_laptop))).execute(&mut pool)?;
            Ok(to_use)
        })
        .await?
    }
    pub async fn delete_batch(
        &self,
        ctx: &Context<'_>,
        ids: Vec<Uuid>,
    ) -> crate::Result<Vec<Laptop>> {
        let mut pool = ctx.pool()?;
        block(move || -> crate::Result<Vec<Laptop>> {
            use crate::schema::laptop::dsl::*;
            let to_use = Self::get_laptops(&ids, &mut pool)?;
            diesel::delete(
                laptop.filter(
                    id_laptop.eq_any(&to_use.iter().map(|t| t.id_laptop).collect::<Vec<_>>()),
                ),
            )
            .execute(&mut pool)?;
            Ok(to_use)
        })
        .await?
    }
}
