use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub enum LoanState {
    Pending,
    Approved,
    Failed,
}

#[derive(Serialize, Clone)]
pub struct Loan {
    pub customer_number: String,
    pub state: LoanState,
}

#[derive(Deserialize)]
pub struct SubscribeRequest {
    pub customer_number: String,
}

#[derive(Serialize)]
pub struct SubscribeResponse {
    pub loan_id: Uuid,
    pub state: LoanState,
}

#[derive(Deserialize)]
pub struct LoanRequest {
    pub customer_number: String,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct LoanRequestResponse {
    pub loan_id: Uuid,
    pub state: LoanState,
    pub score: Option<u32>,
    pub limit_amount: Option<f64>,
}
