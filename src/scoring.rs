use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use blake3;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub type ScoringError = Box<dyn std::error::Error + Send + Sync>;

pub async fn get_score(national_id: &str) -> Result<(u32, f64), ScoringError> {
    let use_real = std::env::var("USE_REAL_SCORING")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if use_real {
        get_real_score(national_id).await
    } else {
        Ok((mock_score(national_id), -1.0))
    }
}

fn mock_score(national_id: &str) -> u32 {
    let seed: [u8; 32] = *blake3::hash(national_id.as_bytes()).as_bytes();
    let mut rng = StdRng::from_seed(seed);
    500 + rng.gen_range(0..300)
}

async fn get_real_score(national_id: &str) -> Result<(u32, f64), ScoringError> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let base = "https://scoringtest.credable.io/api/v1";

    let init_url = format!("{}/scoring/initiateQueryScore/{}", base, national_id);
    let token_resp = client
        .get(&init_url)
        .header("client-token", "")
        .send()
        .await?
        .error_for_status()?;
    let token: String = token_resp.json().await?;

    let mut attempts = 0;
    loop {
        let query_url = format!("{}/scoring/queryScore/{}", base, token);
        let r = client
            .get(&query_url)
            .header("client-token", token.clone())
            .send()
            .await?;

        if r.status().is_success() {
            let body: Value = r.json().await?;
            let score = body
                .get("score")
                .and_then(Value::as_u64)
                .unwrap_or(0) as u32;
            let limit = body
                .get("limitAmount")
                .and_then(Value::as_f64)
                .unwrap_or(0.0);
            return Ok((score, limit));
        }

        attempts += 1;
        if attempts >= 5 {
            return Err("scoring engine retry timeout".into());
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
