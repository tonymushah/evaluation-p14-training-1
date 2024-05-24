use crate::{generate_crud_mutation, models::type_ram::TypeRam};

use diesel::prelude::*;
use uuid::Uuid;

generate_crud_mutation! {
    #[mutation] TypeRamCrudMutations,
    #[base] TypeRam,
    #[dsl] crate::schema::type_ram::dsl,
    #[table] type_ram,
    #[id] id_type => Uuid,
    #[input] TypeRam
}
