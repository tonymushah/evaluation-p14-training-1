use crate::{generate_upserts, models::marque::Marque};

use diesel::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct MarqueMutations;

generate_upserts!(
    MarqueMutations,
    Marque,
    Marque,
    marque,
    id_marque,
    crate::schema::marque::dsl
);
