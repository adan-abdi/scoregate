use crate::models::{Loan, LoanState, SubscribeRequest, SubscribeResponse};
use dashmap::DashMap;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use warp::{http::StatusCode, reply, Rejection, Reply};

pub type LoanStore = Arc<DashMap<Uuid, Loan>>;

pub async fn subscribe_handler(
    req: SubscribeRequest,
    store: LoanStore,
) -> Result<impl Reply, Rejection> {
    if req.customer_number.trim().is_empty() {
        let err = json!({ "error": "customer_number cannot be empty" });
        return Ok(reply::with_status(reply::json(&err), StatusCode::BAD_REQUEST));
    }

    tracing::info!("New subscription request for customer {}", req.customer_number);

    if store.iter().any(|r| r.value().customer_number == req.customer_number
        && matches!(r.value().state, LoanState::Pending))
    {
        let err = json!({ "error": "existing pending loan" });
        return Ok(reply::with_status(reply::json(&err), StatusCode::CONFLICT));
    }

    let loan_id = Uuid::new_v4();
    let loan = Loan {
        customer_number: req.customer_number.clone(),
        state: LoanState::Pending,
    };
    store.insert(loan_id, loan);

    let resp = SubscribeResponse {
        loan_id,
        state: LoanState::Pending,
    };

    Ok(reply::with_status(reply::json(&resp), StatusCode::CREATED))
}
