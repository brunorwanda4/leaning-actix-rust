use std::sync::Arc;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use utils::{app_state::AppState, database::db_conn::ConnDb};

mod error;
mod model;
mod routes;
mod utils;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = *utils::constants::PORT;
    let address = (*utils::constants::ADDRESS).clone();
    let app_state = Arc::new(AppState {
        db: ConnDb::init().await.unwrap().into(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(app_state.clone()))
            .wrap(Logger::default())
            .configure(routes::home_route::config)
    })
    .bind((address, port))?
    .run()
    .await
}
