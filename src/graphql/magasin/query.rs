use async_graphql::Object;

#[derive(Debug, Clone, Copy, Default)]
pub struct MagasinQueries;

#[Object]
impl MagasinQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from magasin")
    }
}
