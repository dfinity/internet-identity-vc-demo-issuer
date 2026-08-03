//! Bindings for Internet Identity's verifiable-credentials (id alias) methods.
//!
//! Vendored from `dfinity/internet-identity`,
//! `src/canister_tests/src/api/internet_identity/vc_mvp.rs`.

use crate::types::{
    CanisterId, GetIdAliasError, GetIdAliasRequest, IdAliasCredentials, PrepareIdAliasError,
    PrepareIdAliasRequest, PreparedIdAlias,
};
use candid::Principal;
use pocket_ic::common::rest::RawEffectivePrincipal;
use pocket_ic::{call_candid_as, query_candid_as, PocketIc, RejectResponse};

pub fn prepare_id_alias(
    env: &PocketIc,
    canister_id: CanisterId,
    sender: Principal,
    prepare_id_alias_req: PrepareIdAliasRequest,
) -> Result<Result<PreparedIdAlias, PrepareIdAliasError>, RejectResponse> {
    call_candid_as(
        env,
        canister_id,
        RawEffectivePrincipal::None,
        sender,
        "prepare_id_alias",
        (prepare_id_alias_req,),
    )
    .map(|(x,)| x)
}

pub fn get_id_alias(
    env: &PocketIc,
    canister_id: CanisterId,
    sender: Principal,
    get_id_alias_req: GetIdAliasRequest,
) -> Result<Result<IdAliasCredentials, GetIdAliasError>, RejectResponse> {
    query_candid_as(
        env,
        canister_id,
        sender,
        "get_id_alias",
        (get_id_alias_req,),
    )
    .map(|(x,)| x)
}
