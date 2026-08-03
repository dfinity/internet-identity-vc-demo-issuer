//! Reusable flows consisting of multiple canister interactions.
//!
//! Vendored from `dfinity/internet-identity`, `src/canister_tests/src/flows.rs`.

use crate::api::{create_challenge, register};
use crate::framework::{device_data_1, principal_1};
use crate::types::{AnchorNumber, CanisterId, ChallengeAttempt, DeviceData, RegisterResponse};
use candid::Principal;
use pocket_ic::PocketIc;

pub fn register_anchor(env: &PocketIc, canister_id: CanisterId) -> AnchorNumber {
    register_anchor_with(env, canister_id, principal_1(), &device_data_1())
}

pub fn register_anchor_with_device(
    env: &PocketIc,
    canister_id: CanisterId,
    device: &DeviceData,
) -> AnchorNumber {
    register_anchor_with(env, canister_id, device.principal(), device)
}

pub fn register_anchor_with(
    env: &PocketIc,
    canister_id: CanisterId,
    sender: Principal,
    device_data: &DeviceData,
) -> AnchorNumber {
    let challenge = create_challenge(env, canister_id).expect("challenge creation failed");
    match register(
        env,
        canister_id,
        sender,
        device_data,
        &ChallengeAttempt {
            chars: "a".to_string(),
            key: challenge.challenge_key,
        },
        None,
    ) {
        Ok(RegisterResponse::Registered { user_number }) => user_number,
        response => panic!("could not register: {response:?}"),
    }
}
