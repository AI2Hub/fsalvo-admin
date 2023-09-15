use std::env;

use log::info;
use salvo::affix;
use salvo::prelude::*;
use sea_orm::{Database, DatabaseConnection};

use crate::handler::menu_handler::{*};
use crate::handler::role_handler::{*};
use crate::handler::user_handler::{*};
use crate::utils::auth::auth_token;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;


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
    let router = Router::new().hoop(affix::inject(state)).path("/api").get(hello)
        .push(Router::new().path("login").post(login))
        .push(
            Router::new().hoop(auth_token)
                .push(Router::new().path("query_user_role").post(query_user_role))
                .push(Router::new().path("update_user_role").post(update_user_role))
                .push(Router::new().path("query_user_menu").get(query_user_menu))
                .push(Router::new().path("user_list").post(user_list))
                .push(Router::new().path("user_save").post(user_save))
                .push(Router::new().path("user_update").post(user_update))
                .push(Router::new().path("user_delete").post(user_delete))
                .push(Router::new().path("update_user_password").post(update_user_password))
                .push(Router::new().path("role_list").post(role_list))
                .push(Router::new().path("role_save").post(role_save))
                .push(Router::new().path("role_update").post(role_update))
                .push(Router::new().path("role_delete").post(role_delete))
                .push(Router::new().path("query_role_menu").post(query_role_menu))
                .push(Router::new().path("update_role_menu").post(update_role_menu))
                .push(Router::new().path("menu_list").post(menu_list))
                .push(Router::new().path("menu_save").post(menu_save))
                .push(Router::new().path("menu_update").post(menu_update))
                .push(Router::new().path("menu_delete").post(menu_delete))
        );

    let acceptor = TcpListener::new("0.0.0.0:8100").bind().await;
    Server::new(acceptor).serve(router).await;
}
