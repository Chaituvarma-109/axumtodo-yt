mod routes;
mod handlers;
mod model;
mod schema;

use crate::{
    routes::create_router,
};

use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use axum::{
    http::header::{ACCEPT, CONTENT_TYPE},
    http::{HeaderValue, Method},
    handler::Handler,
};
use sqlx::{
    postgres::PgPoolOptions,
    Pool,
    Postgres
};

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to read .env file");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new().max_connections(6).connect(&database_url).await{
        Ok(pool) => {
            println!("Successfully connected to database");
            pool
        },
        Err(err) => {
            println!("Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([ACCEPT, CONTENT_TYPE]);

    let app = create_router(AppState{ db: pool.clone() }).await.layer(cors);
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.expect("Failed to run server");
}
