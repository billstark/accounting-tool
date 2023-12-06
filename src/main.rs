#![allow(unused)]

mod models;
mod web;
mod errors;

use axum::{Router, response::Html, routing::get};
use dotenv::dotenv;
use log::info;
use sea_orm::{Database, ConnectOptions, DatabaseConnection};
use std::{net::SocketAddr, env};
use migration::{Migrator, MigratorTrait};

pub use self::errors::{Error, Result};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv().ok();
    let db_host = env::var("DB_HOST").unwrap();
    let db_port = env::var("DB_PORT").unwrap();
    let db_username = env::var("DB_USERNAME").unwrap();
    let db_password = env::var("DB_PASSWORD").unwrap();

    let conn_str = format!("postgres://{}:{}@{}:{}/accounting", db_username, db_password, db_host, db_port);
    let mut opt = ConnectOptions::new(conn_str);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await.expect("Error connecting to DB");
    tracing::info!("DB connection ok");
    
    migration::Migrator::up(&db, None).await;
    tracing::info!("DB migration ok");
    
    let state = AppState { conn: db };
    let router = Router::new().merge(web::routes_txn_types::routes(state));

    tracing::info!("Routing setup ok");
    tracing::info!("Listening to 0.0.0.0:8000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}