//! Rust bindings for the canister methods the issuer tests call, derived
//! (manually) from Internet Identity's Candid file.
//!
//! Vendored from `dfinity/internet-identity`, `src/canister_tests/src/api`.

use crate::types::{
    CanisterId, Challenge, ChallengeAttempt, DeviceData, HttpRequest, HttpResponse,
    RegisterResponse,
};
use candid::Principal;
use pocket_ic::common::rest::RawEffectivePrincipal;
use pocket_ic::{call_candid, call_candid_as, query_candid, PocketIc, RejectResponse};

/// API of the verifiable credentials MVP.
pub mod vc_mvp;

// ── methods common to all canisters ─────────────────────────────────────────

pub fn http_request(
    env: &PocketIc,
    canister_id: CanisterId,
    http_request: &HttpRequest,
) -> Result<HttpResponse, RejectResponse> {
    query_candid(env, canister_id, "http_request", (http_request,)).map(|(x,)| x)
}

// ── Internet Identity ───────────────────────────────────────────────────────

pub fn create_challenge(
    env: &PocketIc,
    canister_id: CanisterId,
) -> Result<Challenge, RejectResponse> {
    call_candid(
        env,
        canister_id,
        RawEffectivePrincipal::None,
        "create_challenge",
        (),
    )
    .map(|(x,)| x)
}

pub fn register(
    env: &PocketIc,
    canister_id: CanisterId,
    sender: Principal,
    device_data: &DeviceData,
    challenge_attempt: &ChallengeAttempt,
    temp_key: Option<Principal>,
) -> Result<RegisterResponse, RejectResponse> {
    call_candid_as(
        env,
        canister_id,
        RawEffectivePrincipal::None,
        sender,
        "register",
        (device_data, challenge_attempt, temp_key),
    )
    .map(|(x,)| x)
}
