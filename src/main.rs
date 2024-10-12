mod routes;

use dotenvy::dotenv;
use std::env;

use deadpool_diesel::postgres::{Manager, Pool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connect to db
    let manager = Manager::new(db_url.to_string(), deadpool_diesel::Runtime::Tokio1);

    let pool = Pool::builder(manager).build().unwrap();

    let _ = pool.get().await.unwrap();

    // start server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes::handler_hello().into_make_service())
        .await
        .unwrap();
}
