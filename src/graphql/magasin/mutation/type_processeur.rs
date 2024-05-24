use crate::{generate_crud_mutation, models::type_processeur::TypeProcesseur};

use diesel::prelude::*;
use uuid::Uuid;

generate_crud_mutation! {
    #[mutation] TypeProcesseurCrudMutations,
    #[base] TypeProcesseur,
    #[dsl] crate::schema::type_processeur::dsl,
    #[table] type_processeur,
    #[id] id_type_processeur => Uuid,
    #[input] TypeProcesseur
}
