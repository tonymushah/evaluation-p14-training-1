use async_graphql::{InputObject, SimpleObject};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{schema::ram_ref_laptop, views::v_ram_ref_laptop};

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
#[graphql(input_name = "RamRefLaptopInput")]
#[diesel(table_name = ram_ref_laptop)]
#[diesel(primary_key(id_ram_ref))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RamRefLaptop {
    pub id_ram_ref: Uuid,
    pub marque: Uuid,
    pub puissance: BigDecimal,
    pub type_ram: Uuid,
    pub frequence: BigDecimal,
    pub ref_laptop: Uuid,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_ram_ref_laptop)]
#[diesel(primary_key(ref_laptop))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VRamRefLaptop {
    pub ref_laptop: Uuid,
    pub puissance: BigDecimal,
    pub frequence: BigDecimal,
}
