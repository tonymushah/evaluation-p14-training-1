use actix_cors::Cors;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use dotenvy::dotenv;
use evaluation_p14_training::{
    graphql::{
        magasin::{magasin, magasin_graphiql},
        point_vente::{point_vente, point_vente_graphiql},
    },
    ServerState,
};

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let state = web::Data::new(ServerState::default());
    let port: u16 = env::var("BACKEND_PORT").unwrap().parse().unwrap();
    let adress = env::var("BACKEND_HOST").unwrap();
    println!("Server started at http://{adress}:{port}");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .service(magasin)
            .service(magasin_graphiql)
            .service(point_vente)
            .service(point_vente_graphiql)
    })
    .bind((adress, port))?
    .run()
    .await
}
