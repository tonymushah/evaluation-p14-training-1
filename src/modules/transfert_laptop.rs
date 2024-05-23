use async_graphql::ErrorExtensions;
use uuid::Uuid;

use crate::DbPoolConnection;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("this laptop was already transfered to {0:?}")]
    AlreadyTransfered(Option<Uuid>),
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{self}")).extend_with(|_err, e| match self {
            Error::AlreadyTransfered(er) => {
                e.set("code", "LAPTOP_ALREADY_TRANSFERED");
                if let Some(lap) = er {
                    e.set("to", lap.to_string());
                }
            }
        })
    }
}

pub struct TransfertLaptopModule<'a> {
    pub laptop_id: Uuid,
    pub connection: &'a mut DbPoolConnection,
}

impl<'a> TransfertLaptopModule<'a> {}
