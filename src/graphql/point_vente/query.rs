use async_graphql::Object;

#[derive(Debug, Clone, Copy, Default)]
pub struct PointVenteQueries;

#[Object]
impl PointVenteQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from magasin")
    }
}
