mod db;
mod domain;
mod handlers;
mod repository;
mod routes;

use axum::Router;
use dotenvy::dotenv;
use std::{env, sync::Arc};

use deadpool_diesel::postgres::{Manager, Pool};
use tokio::net::TcpListener;

struct AppState {
    db: Pool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connect to db
    let manager = Manager::new(db_url.to_string(), deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    // // start server
    let shared_state = Arc::new(AppState { db: pool });

    let routes = Router::new().merge(routes::all_routes(shared_state));

    // start server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
