mod crud;
mod dto;

use crate::dto::Item;
use axum::Router;
use axum::routing::get;
use std::sync::{Arc, Mutex};

type SharedState = Arc<Mutex<Vec<Item>>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/items", get(crud::list_items).post(crud::create_item))
        .route(
            "/items/:id",
            get(crud::get_item)
                .put(crud::update_item)
                .delete(crud::delete_item),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
