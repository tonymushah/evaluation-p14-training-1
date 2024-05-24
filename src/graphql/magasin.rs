use std::ops::Deref;

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use self::{mutation::MagasinMutations, query::MagasinQueries};
use crate::ServerState;

pub mod mutation;
pub mod query;

type MagasinSchemaInner = Schema<MagasinQueries, MagasinMutations, EmptySubscription>;

#[derive(Default, Clone)]
pub struct MagasinSchema(MagasinSchemaInner);

impl Deref for MagasinSchema {
    type Target = MagasinSchemaInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[post("/magasin")]
pub async fn magasin(
    state: web::Data<ServerState>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let request = gql_request
        .into_inner()
        .data(state.db.clone())
        .data(req.headers().clone());
    state.magasin.execute(request).await.into()
}

#[get("/magasin")]
pub async fn magasin_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/magasin").finish()))
}
