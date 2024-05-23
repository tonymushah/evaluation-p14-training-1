use async_graphql::ErrorExtensions;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    models::{point_vente_stock::PointVenteStockEntry, ventes::Vente},
    DbPoolConnection,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("this {} laptop is not in the waiting list", .0.to_string())]
    NotInWaitingList(Uuid),
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            Error::NotInWaitingList(_) => e.set("code", "LAPTOP_NOT_IN_THE_WAITING_LIST"),
        })
    }
}

pub struct ValidateTransfertModule<'a> {
    pub point_vente: Uuid,
    pub conn: &'a mut DbPoolConnection,
}

impl<'a> ValidateTransfertModule<'a> {}
