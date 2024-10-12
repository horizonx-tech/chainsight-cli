use std::{thread::sleep, time::Duration};

use anyhow::Result;
use semver::Op;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CanisterResponse {
    pub canister_id: String,
    pub controllers: Vec<String>,
    pub enabled: bool,
    pub id: u64,
    pub module_hash: String,
    pub name: String,
    pub subnet_id: String,
    pub updated_at: String,
}

const BASE_URL: &str = "https://ic-api.internetcomputer.org/api/v3";

pub async fn get_canister(canister_id: &str) -> Result<CanisterResponse, reqwest::Error> {
    reqwest::get(format!("{}/canisters/{}", BASE_URL, canister_id))
        .await?
        .json::<CanisterResponse>()
        .await
}

pub async fn get_canister_with_retry(
    canister_id: &str,
    max_retry: Option<u8>,
) -> Result<CanisterResponse, reqwest::Error> {
    let mut retry = 0;
    let max_retry = max_retry.unwrap_or(3);
    loop {
        match get_canister(canister_id).await {
            Ok(response) => return Ok(response),
            Err(e) => {
                if retry >= max_retry {
                    return Err(e);
                }
                retry += 1;
                sleep(Duration::from_secs(1));
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn test_get_canister() {
        let canister_id = "klzmf-pqaaa-aaaao-a3taa-cai";
        let response = get_canister(canister_id).await.unwrap();
        println!("{:?}", response);
        assert_eq!(response.canister_id, canister_id);
    }
}
