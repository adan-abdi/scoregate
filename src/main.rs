mod models;
mod routes;
mod scoring;
use routes::subscribe::subscribe_handler;
use routes::loan_request::loan_request_handler;
use routes::subscribe::LoanStore;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use dashmap::DashMap;
use tokio::main;
use tracing_subscriber;
use warp::{Filter};


#[main]
async fn main() {
    dotenv().ok();

    let use_real = env::var("USE_REAL_SCORING")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    println!("USE_REAL_SCORING = {}", use_real);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let store: LoanStore = Arc::new(DashMap::new());
    let store_filter = warp::any().map(move || store.clone());

    let root = warp::path::end().map(|| "Welcome to scoregate 🦀");

    let subscribe = warp::path("subscribe")
        .and(warp::post())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(subscribe_handler);

    let loan_request = warp::path("loan-request")
        .and(warp::post())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(loan_request_handler);


    let routes = root
        .or(subscribe)
        .or(loan_request);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
