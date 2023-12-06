#![allow(unused)]

use axum::{Router, response::Html, routing::get};
use dotenv::dotenv;
use log::info;
use sea_orm::{Database, ConnectOptions};
use std::{net::SocketAddr, env};
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/hello", get(|| async {
        Html("Hello <strong>World<strong>")
    }));

    dotenv().ok();

    let db_host = env::var("DB_HOST").unwrap();
    let db_port = env::var("DB_PORT").unwrap();
    let db_username = env::var("DB_USERNAME").unwrap();
    let db_password = env::var("DB_PASSWORD").unwrap();

    let conn_str = format!("postgres://{}:{}@{}:{}/accounting", db_username, db_password, db_host, db_port);
    let mut opt = ConnectOptions::new(conn_str);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await.expect("Error connecting to DB");

    migration::Migrator::up(&db, None).await;
    info!("info: DB connection OK");
    println!("print: DB connection is OK");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
