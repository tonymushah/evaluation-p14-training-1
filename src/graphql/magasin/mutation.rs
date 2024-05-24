use async_graphql::Object;

#[derive(Debug, Clone, Copy, Default)]
pub struct MagasinMutations;

#[Object]
impl MagasinMutations {
    pub async fn test(&self) -> String {
        String::from("some")
    }
}

#[macro_export]
macro_rules! generate_upserts {
    ($parent:ty, $input: ty, $output: ty, $table: expr, $id: expr, $dsl: path) => {
        #[async_graphql::Object]
        impl $parent {
            pub async fn upsert_data(
                &self,
                ctx: &async_graphql::Context<'_>,
                input: $input,
            ) -> $crate::Result<$output> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<$output> {
                    use $dsl::*;
                    let to_input: $output = input.into();
                    diesel::insert_into($table)
                        .values(&to_input)
                        .on_conflict($id)
                        .do_update()
                        .set(&to_input)
                        .get_results(&mut pool)?
                        .first()
                        .cloned()
                        .ok_or($crate::Error::UpsertNotFound)
                })
                .await?
            }
            pub async fn upsert_data_batch(
                &self,
                ctx: &async_graphql::Context<'_>,
                input: Vec<$input>,
            ) -> $crate::Result<Vec<$output>> {
                let mut pool = $crate::graphql::get_pool(ctx)?;
                actix_web::web::block(move || -> $crate::Result<Vec<$output>> {
                    use $dsl::*;
                    let to_input: Vec<$output> = input.into_iter().map(|i| i.into()).collect();
                    let mut res = diesel::insert_into($table)
                        .values(&to_input)
                        .on_conflict($id)
                        .do_nothing()
                        .get_results(&mut pool)?;
                    for i in &to_input {
                        res.append(&mut diesel::update($table).set(i).get_results(&mut pool)?);
                    }
                    Ok(res)
                })
                .await?
            }
        }
    };
}
