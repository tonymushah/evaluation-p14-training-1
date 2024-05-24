use crate::{generate_crud_mutation, models::type_clavier::TypeClavier};

use diesel::prelude::*;
use uuid::Uuid;

generate_crud_mutation! {
    #[mutation] TypeClavierCrudMutations,
    #[base] TypeClavier,
    #[dsl] crate::schema::type_clavier::dsl,
    #[table] type_clavier,
    #[id] id_type_clavier => Uuid,
    #[input] TypeClavier
}
