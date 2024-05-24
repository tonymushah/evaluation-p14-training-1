use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{graphql::GetPoolConnection, schema::laptop, views::v_laptop};

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
)]
#[graphql(complex)]
#[diesel(table_name = v_laptop)]
#[diesel(primary_key(id_laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Laptop {
    pub id_laptop: Uuid,
    #[graphql(skip)]
    pub type_processeur: Uuid,
    #[graphql(skip)]
    pub carte_graphique: Uuid,
    pub ecran: BigDecimal,
    #[graphql(skip)]
    pub type_clavier: Uuid,
    pub prix_unitaire: BigDecimal,
    pub ref_laptop: Uuid,
}

#[ComplexObject]
impl Laptop {
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
    InputObject,
)]
#[diesel(table_name = laptop)]
#[diesel(primary_key(id_laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LaptopInput {
    pub id_laptop: Uuid,
    pub type_processeur: Option<Uuid>,
    pub carte_graphique: Option<Uuid>,
    pub ecran: Option<BigDecimal>,
    pub type_clavier: Option<Uuid>,
    pub prix_unitaire: Option<BigDecimal>,
    pub ref_laptop: Uuid,
}
