use crate::{generate_crud_mutation, models::reference_laptop::ReferenceLaptop};

use diesel::prelude::*;
use uuid::Uuid;

generate_crud_mutation! {
    #[mutation] ReferenceLaptopCrudMutations,
    #[base] ReferenceLaptop,
    #[dsl] crate::schema::reference_laptop::dsl,
    #[table] reference_laptop,
    #[id] id_ref => Uuid,
    #[input] ReferenceLaptop
}
