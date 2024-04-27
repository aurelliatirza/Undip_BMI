use serde::{Deserialize, Serialize};
use candid::CandidType; // Tambahkan import untuk CandidType

// Tambahkan macro derive untuk CandidType pada struct UserData
#[derive(CandidType, Deserialize, Serialize)]
struct UserData {
    height: f64,
    weight: f64,
}

#[ic_cdk::query]
fn calculate_bmi(height: f64, weight: f64) -> f64 {
    let bmi = weight / (height * height);
    bmi
}

#[ic_cdk::export_service] // Ganti export_candid menjadi export_service
mod icp_rust_boilerplate_backend {
    #[ic_cdk::update]
    fn add_user_data(data: UserData) -> f64 {
        let bmi = calculate_bmi(data.height, data.weight);
        bmi
    }
}

// Skrip untuk menghasilkan file DID
#!/usr/bin/env bash

function generate_did() {
  local canister=$1
  canister_root="src/$canister"

  cargo build --manifest-path="$canister_root/Cargo.toml" \
      --target wasm32-unknown-unknown \
      --release --package "$canister" \

  candid-extractor "target/wasm32-unknown-unknown/release/$canister.wasm" > "$canister_root/$canister.did"
}

CANISTERS=icp_rust_boilerplate_backend

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    generate_did "$canister"
done
