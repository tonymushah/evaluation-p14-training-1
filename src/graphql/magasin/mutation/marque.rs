use crate::{generate_crud_mutation, models::marque::Marque};

use diesel::prelude::*;
use uuid::Uuid;

generate_crud_mutation! {
    #[mutation] MarqueCrudMutations,
    #[base] Marque,
    #[dsl] crate::schema::marque::dsl,
    #[table] marque,
    #[id] id_marque => Uuid,
    #[input] Marque
}
