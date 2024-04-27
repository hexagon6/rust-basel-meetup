mod controllers;
mod database;
use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use controllers::{add_item, delete_item, get_items};
use database::InMemoryDatabase;
use model::ShoppingListItem;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;

type Database = Arc<RwLock<InMemoryDatabase>>;

#[derive(Serialize, Deserialize)]
struct Workshop {
    attendees_count: i32,
    people_like_it: bool,
}

#[tokio::main]
async fn main() {
    let db = Database::default();
    let app = Router::new()
        .route("/items", get(get_items).post(add_item))
        .route("/items/:uuid", delete(delete_item))
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
