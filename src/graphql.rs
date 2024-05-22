use async_graphql::Context;

use crate::{DbPool, DbPoolConnection, ServerState};

pub mod magasin;
pub mod point_vente;

pub trait GetPoolConnection {
    fn pool(&self) -> crate::Result<DbPoolConnection>;
}

impl<'a> GetPoolConnection for Context<'a> {
    fn pool(&self) -> crate::Result<DbPoolConnection> {
        Ok(self
            .data::<DbPool>()
            .map_err(crate::Error::GraphQL)?
            .get()?)
    }
}

impl GetPoolConnection for ServerState {
    fn pool(&self) -> crate::Result<DbPoolConnection> {
        Ok(self.db.get()?)
    }
}
