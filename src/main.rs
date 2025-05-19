mod models;
mod routes;

use routes::subscribe::subscribe_handler;
use std::sync::Arc;
use dashmap::DashMap;
use tokio::main;
use tracing_subscriber;
use warp::{Filter};

use models::Loan;

#[main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let store: Arc<DashMap<uuid::Uuid, Loan>> = Arc::new(DashMap::new());
    let store_filter = warp::any().map(move || store.clone());

    let root = warp::path::end().map(|| "Welcome to scoregate 🦀");

    let subscribe = warp::path("subscribe")
        .and(warp::post())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(subscribe_handler);

    let routes = root.or(subscribe);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
