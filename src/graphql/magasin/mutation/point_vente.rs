use crate::{generate_crud_mutation, models::point_vente::PointVente};

use diesel::prelude::*;
use uuid::Uuid;

generate_crud_mutation! {
    #[mutation] PointVenteCrudMutations,
    #[base] PointVente,
    #[dsl] crate::schema::point_vente::dsl,
    #[table] point_vente,
    #[id] id_point_vente => Uuid,
    #[input] PointVente
}
