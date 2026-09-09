use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use crate::dto::{CreateItem, Item, UpdateItem};
use crate::SharedState;

pub async fn create_item(
    State(state): State<SharedState>,
    Json(payload): Json<CreateItem>,
) -> (StatusCode, Json<Item>) {
    let item = Item {
        id: Uuid::new_v4(),
        name: payload.name,
        done: false,
    };

    let mut items = state.lock().unwrap();
    items.push(item.clone());

    (StatusCode::CREATED, Json(item))
}

pub async fn list_items(State(state): State<SharedState>) -> Json<Vec<Item>> {
    let items = state.lock().unwrap();
    Json(items.clone())
}

pub async fn get_item(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Item>, StatusCode> {
    let items = state.lock().unwrap();
    items
        .iter()
        .find(|i| i.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
pub async fn update_item(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateItem>,
) -> Result<Json<Item>, StatusCode> {
    let mut items = state.lock().unwrap();
    let item = items.iter_mut().find(|i| i.id == id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(name) = payload.name {
        item.name = name;
    }
    if let Some(done) = payload.done {
        item.done = done;
    }

    Ok(Json(item.clone()))
}

// DELETE
pub async fn delete_item(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut items = state.lock().unwrap();
    let len_before = items.len();
    items.retain(|i| i.id != id);

    if items.len() < len_before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}