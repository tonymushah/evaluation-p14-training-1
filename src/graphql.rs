use async_graphql::{Context, InputObject, SimpleObject};
use diesel::{query_dsl::methods::LoadQuery, PgConnection, QueryResult};

use crate::{models::Paginated, DbPool, DbPoolConnection, ServerState};

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

#[derive(Debug, Clone, Copy, InputObject)]
pub struct OffsetLimit {
    pub offset: i64,
    pub limit: i64,
}

impl Default for OffsetLimit {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 10,
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct ResultsData<T>
where
    T: async_graphql::OutputType,
{
    pub data: Vec<T>,
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

impl<T> Paginated<T> {
    pub fn to_results_data<'a, U>(self, conn: &mut PgConnection) -> QueryResult<ResultsData<U>>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
        U: async_graphql::OutputType,
    {
        let offset = self.get_offset();
        let limit = self.get_limit();
        let (data, total) = self.load_and_count_pages(conn)?;
        Ok(ResultsData {
            data,
            limit,
            offset,
            total,
        })
    }
}

pub(crate) fn get_pool(ctx: &async_graphql::Context<'_>) -> crate::Result<crate::DbPoolConnection> {
    Ok(ctx
        .data::<crate::DbPool>()
        .map_err(crate::Error::GraphQL)?
        .get()?)
}
