use crate::models::{Loan, LoanRequest, LoanRequestResponse, LoanState};
use crate::routes::subscribe::LoanStore;
use crate::scoring::get_score;
use serde_json::json;
use warp::{http::StatusCode, reply, Rejection, Reply};

pub async fn loan_request_handler(
    req: LoanRequest,
    store: LoanStore,
) -> Result<impl Reply, Rejection> {
    if req.customer_number.trim().is_empty() || req.amount <= 0.0 {
        let err = json!({ "error": "invalid input" });
        return Ok(reply::with_status(reply::json(&err), StatusCode::BAD_REQUEST));
    }

    let has_pending = store.iter().any(|r| {
        let loan = r.value();
        loan.customer_number == req.customer_number && matches!(loan.state, LoanState::Pending)
    });
    if !has_pending {
        let err = json!({ "error": "no subscription found" });
        return Ok(reply::with_status(reply::json(&err), StatusCode::NOT_FOUND));
    }

    let (score, real_limit) = match get_score(&req.customer_number).await {
        Ok(tuple) => tuple,
        Err(_) => {
            let err = json!({ "error": "scoring service unreachable" });
            return Ok(reply::with_status(reply::json(&err), StatusCode::BAD_GATEWAY));
        }
    };

    let limit_amount = if std::env::var("USE_REAL_SCORING")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        Some(real_limit)
    } else {
        Some((req.amount * 1.5).min(10_000.0))
    };

    let loan_id = store
        .iter()
        .find_map(|r| {
            let (id, loan) = (r.key(), r.value());
            if loan.customer_number == req.customer_number
                && matches!(loan.state, LoanState::Pending)
            {
                Some(*id)
            } else {
                None
            }
        })
        .unwrap();

    let new_state = if score > 500 {
        LoanState::Approved
    } else {
        LoanState::Failed
    };

    store.insert(
        loan_id,
        Loan {
            customer_number: req.customer_number.clone(),
            state: new_state.clone(),
        },
    );

    let resp = LoanRequestResponse {
        loan_id,
        state: new_state,
        score: Some(score),
        limit_amount,
    };

    Ok(reply::with_status(reply::json(&resp), StatusCode::OK))
}
