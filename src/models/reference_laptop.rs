use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{graphql::GetPoolConnection, schema::reference_laptop};

use super::{
    type_carte_graphique::TypeCarteGraphique, type_clavier::TypeClavier,
    type_processeur::TypeProcesseur,
};

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
#[graphql(input_name = "ReferenceLaptopInput", complex)]
#[diesel(table_name = reference_laptop)]
#[diesel(primary_key(id_ref))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReferenceLaptop {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_ref: Uuid,
    #[graphql(skip_output)]
    pub type_processeur: Uuid,
    #[graphql(skip_output)]
    pub carte_graphique: Uuid,
    pub ecran: BigDecimal,
    #[graphql(skip_output)]
    pub type_clavier: Uuid,
    pub prix_unitaire: BigDecimal,
}

#[ComplexObject]
impl ReferenceLaptop {
    pub async fn processeur(&self, ctx: &Context<'_>) -> crate::Result<TypeProcesseur> {
        let proc = self.type_processeur;
        ctx.use_pool(move |mut pool| {
            use crate::schema::type_processeur::dsl::*;
            Ok(type_processeur
                .filter(id_type_processeur.eq(proc))
                .select(TypeProcesseur::as_select())
                .get_result(&mut pool)?)
        })
        .await
    }
    pub async fn carte_graphique(&self, ctx: &Context<'_>) -> crate::Result<TypeCarteGraphique> {
        let gpu = self.carte_graphique;
        ctx.use_pool(move |mut pool| {
            use crate::schema::type_carte_graphique::dsl::*;
            Ok(type_carte_graphique
                .filter(id_type.eq(gpu))
                .select(TypeCarteGraphique::as_select())
                .get_result(&mut pool)?)
        })
        .await
    }
    pub async fn clavier(&self, ctx: &Context<'_>) -> crate::Result<TypeClavier> {
        let id = self.type_clavier;
        ctx.use_pool(move |mut pool| {
            use crate::schema::type_clavier::dsl::*;
            Ok(type_clavier
                .filter(id_type_clavier.eq(id))
                .select(TypeClavier::as_select())
                .get_result(&mut pool)?)
        })
        .await
    }
}
