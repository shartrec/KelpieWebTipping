// backend/src/routes/tippers.rs
use rocket::serde::json::Json;
use rocket::State;
use crate::models::tipper::Tipper;
use tokio::sync::Mutex;
use crate::TipperStore;

#[get("/api/tippers")]
pub async fn list(store: &State<TipperStore>) -> Json<Vec<Tipper>> {
    Json(store.lock().await.clone())
}

#[post("/api/tippers", data = "<tipper>")]
pub async fn add(tipper: Json<Tipper>, store: &State<TipperStore>) -> Json<Tipper> {
    let mut tippers = store.lock().await;
    let mut new = tipper.into_inner();
    let next_id = tippers.iter().map(|t| t.tipper_id).max().unwrap_or(0) + 1;
    new.tipper_id = next_id;
    tippers.push(new.clone());
    Json(new)
}

#[put("/api/tippers/<id>", data = "<tipper>")]
pub async fn update(id: i32, tipper: Json<Tipper>, store: &State<TipperStore>) -> Option<Json<Tipper>> {
    let mut store = store.lock().await;
    if let Some(existing) = store.iter_mut().find(|t| t.tipper_id == id) {
        existing.name = tipper.name.clone();
        existing.email = tipper.email.clone();
        Some(Json(existing.clone()))
    } else {
        None
    }
}

#[delete("/api/tippers/<id>")]
pub async fn delete(id: i32, store: &State<TipperStore>) -> &'static str {
    let mut store = store.lock().await;
    store.retain(|t| t.tipper_id != id);
    "OK"
}
