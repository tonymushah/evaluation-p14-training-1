use crate::{generate_crud_mutation, models::type_carte_graphique::TypeCarteGraphique};

use diesel::prelude::*;
use uuid::Uuid;

generate_crud_mutation! {
    #[mutation] TypeCarteGraphiqueCrudMutations,
    #[base] TypeCarteGraphique,
    #[dsl] crate::schema::type_carte_graphique::dsl,
    #[table] type_carte_graphique,
    #[id] id_type => Uuid,
    #[input] TypeCarteGraphique
}
