use std::env;

use diesel::r2d2::{self, ConnectionManager};
use diesel::MysqlConnection;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use salvo::prelude::*;

use crate::middleware::auth::auth_token;
use crate::routes::{build_other_route, build_system_route};
use handler::system::sys_user_handler::*;

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod routes;
pub mod schema;
pub mod utils;
pub mod vo;

#[handler]
async fn hello() -> &'static str {
    "Hello World123123"
}

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub static RB: Lazy<DbPool> = Lazy::new(|| {
    let database_url = env::var("database_url").expect("database_url must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});

#[tokio::main]
async fn main() {
    dotenv().ok();
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    // tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("0.0.0.0:8100").bind().await;
    Server::new(acceptor).serve(route()).await;
}

fn route() -> Router {
    Router::new()
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
