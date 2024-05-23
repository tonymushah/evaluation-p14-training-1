use async_graphql::{InputObject, SimpleObject};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{schema::ram_laptop, views::v_ram_laptop};

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
#[graphql(input_name = "RamLaptopInput")]
#[diesel(table_name = ram_laptop)]
#[diesel(primary_key(id_ram_laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RamLaptop {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_ram_laptop: Uuid,
    pub marque: Uuid,
    pub puissance: BigDecimal,
    pub type_ram: Uuid,
    pub frequence: BigDecimal,
    pub laptop: Uuid,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_ram_laptop)]
#[diesel(primary_key(laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VRamLaptop {
    pub laptop: Uuid,
    pub puissance: BigDecimal,
    pub frequence: BigDecimal,
}
