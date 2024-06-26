pub mod laptop;
pub mod marque;
pub mod point_vente;
pub mod ram_laptop;
pub mod ram_ref_laptop;
pub mod ref_laptop;
pub mod type_carte_graphique;
pub mod type_clavier;
pub mod type_processeur;
pub mod type_ram;

use async_graphql::{Object, SimpleObject};

use self::{
    laptop::LaptopCrudMutations, marque::MarqueCrudMutations, point_vente::PointVenteCrudMutations,
    ram_laptop::RamLaptopCrudMutations, ram_ref_laptop::RamRefLaptopCrudMutations,
    ref_laptop::ReferenceLaptopCrudMutations,
    type_carte_graphique::TypeCarteGraphiqueCrudMutations, type_clavier::TypeClavierCrudMutations,
    type_processeur::TypeProcesseurCrudMutations, type_ram::TypeRamCrudMutations,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct MagasinMutations;

#[derive(Debug, Clone, Copy, Default, SimpleObject)]
pub struct CrudMutations {
    marque: MarqueCrudMutations,
    type_ram: TypeRamCrudMutations,
    type_clavier: TypeClavierCrudMutations,
    type_processeur: TypeProcesseurCrudMutations,
    type_carte_graphique: TypeCarteGraphiqueCrudMutations,
    ram_ref_laptop: RamRefLaptopCrudMutations,
    point_vente: PointVenteCrudMutations,
    ref_laptop: ReferenceLaptopCrudMutations,
    laptop: LaptopCrudMutations,
    ram_laptop: RamLaptopCrudMutations,
}

#[Object]
impl MagasinMutations {
    pub async fn cruds(&self) -> CrudMutations {
        CrudMutations::default()
    }
}

#[macro_export]
macro_rules! generate_crud_mutation {
    {
        #[$mutation:ident] $parent:ident,
        #[$base:ident] $base_:ty,
        #[$dsl:ident] $dsl_:path,
        #[$table:ident] $table_:expr,
        #[$id:ident] $id_:expr => $id_type:ty,
        #[$input:ident] $input_:ty
    } => {
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $parent;
        #[async_graphql::Object]
        impl $parent {
            pub async fn upsert(
                &self,
                ctx: &async_graphql::Context<'_>,
                input: $input_,
            ) -> $crate::Result<$base_> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<$base_> {
                    use $dsl_::*;
                    let to_input: $base_ = input.into();
                    diesel::insert_into($table_)
                        .values(&to_input)
                        .on_conflict($id_)
                        .do_update()
                        .set(&to_input)
                        .returning(<$base_>::as_returning())
                        .get_results(&mut pool)?
                        .first()
                        .cloned()
                        .ok_or($crate::Error::UpsertNotFound)
                })
                .await?
            }
            pub async fn upsert_batch(
                &self,
                ctx: &async_graphql::Context<'_>,
                input: Vec<$input_>,
            ) -> $crate::Result<Vec<$base_>> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<Vec<$base_>> {
                    use $dsl_::*;
                    let to_input: Vec<$base_> = input.into_iter().map(|i| i.into()).collect();
                    let mut res = diesel::insert_into($table_)
                        .values(&to_input)
                        .on_conflict($id_)
                        .do_nothing()
                        .returning(<$base_>::as_returning())
                        .get_results(&mut pool)?;
                    for i in &to_input {
                        res.append(&mut diesel::update($table_).set(i).returning(<$base_>::as_returning()).get_results(&mut pool)?);
                    }
                    Ok(res)
                })
                .await?
            }
            pub async fn delete(
                &self,
                ctx: &async_graphql::Context<'_>,
                id: $id_type,
            ) -> $crate::Result<$base_> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<$base_> {
                    use $dsl_::*;
                    Ok(diesel::delete($table_.filter($id_.eq(id)))
                        .returning(<$base_>::as_returning())
                        .get_result(&mut pool)?)
                })
                .await?
            }
            pub async fn delete_batch(
                &self,
                ctx: &async_graphql::Context<'_>,
                ids: Vec<$id_type>,
            ) -> $crate::Result<Vec<$base_>> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<Vec<$base_>> {
                    use $dsl_::*;
                    let res = diesel::delete($table_.filter($id_.eq_any(&ids)))
                        .returning(<$base_>::as_returning())
                        .get_results(&mut pool)?;
                    Ok(res)
                })
                .await?
            }
        }
    }
}
