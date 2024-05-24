use crate::{generate_crud_mutation, models::ram_laptop::RamLaptop};

use diesel::prelude::*;
use uuid::Uuid;

generate_crud_mutation! {
    #[mutation] RamLaptopCrudMutations,
    #[base] RamLaptop,
    #[dsl] crate::schema::ram_laptop::dsl,
    #[table] ram_laptop,
    #[id] id_ram_laptop => Uuid,
    #[input] RamLaptop
}
