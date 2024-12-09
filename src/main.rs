use std::env;

use log::info;
use salvo::prelude::*;
use sea_orm::{Database, DatabaseConnection};

use crate::routes::{build_other_route, build_system_route};
use handler::system::user_handler::*;
use middleware::auth::auth_token;

mod common;
pub mod handler;
mod middleware;
pub mod model;
mod routes;
pub mod utils;
pub mod vo;

#[handler]
async fn hello() -> &'static str {
    "Hello World123123"
}

#[derive(Debug, Clone)]
struct AppState {
    pub conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    // tracing_subscriber::fmt().init();
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    info!("{:?}", state.conn);

    let acceptor = TcpListener::new("0.0.0.0:8100").bind().await;
    Server::new(acceptor).serve(route(state)).await;
}

fn route(state: AppState) -> Router {
    Router::new()
        .hoop(affix_state::inject(state))
        .path("/api")
        .get(hello)
        .push(Router::new().path("login").post(login))
        .push(
            Router::new()
                .hoop(auth_token)
                .push(build_system_route())
                .push(build_other_route()),
        )
}
