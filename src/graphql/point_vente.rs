pub mod query;

use std::ops::Deref;

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use self::query::PointVenteQueries;
use crate::ServerState;

type PointVenteSchemaInner = Schema<PointVenteQueries, EmptyMutation, EmptySubscription>;

#[derive(Default, Clone)]
pub struct PointVenteSchema(PointVenteSchemaInner);

impl Deref for PointVenteSchema {
    type Target = PointVenteSchemaInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[post("/point-vente")]
pub async fn point_vente(
    state: web::Data<ServerState>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let request = gql_request
        .into_inner()
        .data(state.db.clone())
        .data(req.headers().clone());
    state.point_vente.execute(request).await.into()
}

#[get("/point-vente")]
pub async fn point_vente_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/point-vente").finish()))
}
