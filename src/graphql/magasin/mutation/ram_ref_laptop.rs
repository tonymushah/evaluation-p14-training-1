use crate::{generate_crud_mutation, models::ram_ref_laptop::RamRefLaptop};

use diesel::prelude::*;
use uuid::Uuid;

generate_crud_mutation! {
    #[mutation] RamRefLaptopCrudMutations,
    #[base] RamRefLaptop,
    #[dsl] crate::schema::ram_ref_laptop::dsl,
    #[table] ram_ref_laptop,
    #[id] id_ram_ref => Uuid,
    #[input] RamRefLaptop
}
