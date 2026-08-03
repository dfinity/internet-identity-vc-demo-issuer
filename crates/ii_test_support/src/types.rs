//! The Internet Identity types that cross the II ↔ issuer boundary.
//!
//! These are copied from `dfinity/internet-identity`:
//!  * `http_gateway` from `src/internet_identity_interface/src/http_gateway.rs`
//!  * everything else from
//!    `src/internet_identity_interface/src/internet_identity/types.rs` and
//!    `.../types/vc_mvp.rs`
//!
//! They are a **wire contract**: the Candid encoding produced here has to match
//! what the II canister expects and returns. When II changes any of these
//! definitions, the copies below must be updated in lockstep — the scheduled
//! `integration-tests` workflow runs against II's latest release so a drift
//! shows up as a failing test rather than silently.

use candid::{CandidType, Deserialize, Principal};
use serde_bytes::ByteBuf;
use std::collections::HashMap;

/// Alias for a canister id, mirroring `ic_cdk`'s definition.
pub type CanisterId = Principal;

// ── HTTP gateway ────────────────────────────────────────────────────────────
// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#http-gateway-interface

pub type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
    pub certificate_version: Option<u16>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
    pub upgrade: Option<bool>,
}

// ── Internet Identity core types ────────────────────────────────────────────

pub type AnchorNumber = u64;
pub type IdentityNumber = u64;
pub type CredentialId = ByteBuf;
pub type Aaguid = [u8; 16];
pub type PublicKey = ByteBuf;
pub type DeviceKey = PublicKey;
pub type CanisterSigPublicKeyDer = PublicKey;
pub type FrontendHostname = String;
pub type Timestamp = u64; // in nanos since epoch
pub type ChallengeKey = String;

#[derive(Eq, PartialEq, Clone, Debug, CandidType, Deserialize)]
pub struct DeviceData {
    pub pubkey: DeviceKey,
    pub alias: String,
    pub credential_id: Option<CredentialId>,
    pub aaguid: Option<Aaguid>,
    pub purpose: Purpose,
    pub key_type: KeyType,
    pub protection: DeviceProtection,
    pub origin: Option<String>,
    pub metadata: Option<HashMap<String, MetadataEntry>>,
}

impl DeviceData {
    pub fn principal(&self) -> Principal {
        Principal::self_authenticating(&self.pubkey)
    }

    /// Creates a new device that can be used for test purposes.
    pub fn auth_test_device() -> Self {
        Self {
            pubkey: ByteBuf::from([0; 32]),
            alias: "test device".to_string(),
            credential_id: Some(ByteBuf::from([1; 16])),
            aaguid: None,
            purpose: Purpose::Authentication,
            key_type: KeyType::CrossPlatform,
            protection: DeviceProtection::Unprotected,
            origin: Some("https://id.ai".to_string()),
            metadata: None,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, CandidType, Deserialize)]
pub enum Purpose {
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "authentication")]
    Authentication,
}

#[derive(Eq, PartialEq, Clone, Debug, CandidType, Deserialize)]
pub enum KeyType {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "platform")]
    Platform,
    #[serde(rename = "cross_platform")]
    CrossPlatform,
    #[serde(rename = "seed_phrase")]
    SeedPhrase,
    #[serde(rename = "browser_storage_key")]
    BrowserStorageKey,
}

#[derive(Eq, PartialEq, Clone, Debug, CandidType, Deserialize)]
pub enum DeviceProtection {
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "unprotected")]
    Unprotected,
}

#[derive(Eq, PartialEq, Clone, Debug, CandidType, Deserialize)]
pub enum MetadataEntry {
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "bytes")]
    Bytes(ByteBuf),
    #[serde(rename = "map")]
    Map(HashMap<String, MetadataEntry>),
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct Challenge {
    pub png_base64: String,
    pub challenge_key: ChallengeKey,
}

/// The user's attempt at solving a captcha challenge.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ChallengeAttempt {
    pub chars: String,
    pub key: ChallengeKey,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum RegisterResponse {
    #[serde(rename = "registered")]
    Registered { user_number: AnchorNumber },
    #[serde(rename = "canister_full")]
    CanisterFull,
    #[serde(rename = "bad_challenge")]
    BadChallenge,
}

// ── Verifiable-credentials (id alias) types ─────────────────────────────────

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct SignedIdAlias {
    pub id_alias: Principal,
    pub id_dapp: Principal,
    pub credential_jws: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct PrepareIdAliasRequest {
    #[serde(rename = "identity_number")]
    pub identity_number: IdentityNumber,
    #[serde(rename = "relying_party")]
    pub relying_party: FrontendHostname,
    #[serde(rename = "issuer")]
    pub issuer: FrontendHostname,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct PreparedIdAlias {
    #[serde(rename = "canister_sig_pk_der")]
    pub canister_sig_pk_der: CanisterSigPublicKeyDer,
    #[serde(rename = "rp_id_alias_jwt")]
    pub rp_id_alias_jwt: String,
    #[serde(rename = "issuer_id_alias_jwt")]
    pub issuer_id_alias_jwt: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum PrepareIdAliasError {
    Unauthorized(Principal),
    InternalCanisterError(String),
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct IdAliasCredentials {
    pub rp_id_alias_credential: SignedIdAlias,
    pub issuer_id_alias_credential: SignedIdAlias,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct GetIdAliasRequest {
    #[serde(rename = "identity_number")]
    pub identity_number: IdentityNumber,
    #[serde(rename = "relying_party")]
    pub relying_party: FrontendHostname,
    #[serde(rename = "issuer")]
    pub issuer: FrontendHostname,
    #[serde(rename = "rp_id_alias_jwt")]
    pub rp_id_alias_jwt: String,
    #[serde(rename = "issuer_id_alias_jwt")]
    pub issuer_id_alias_jwt: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum GetIdAliasError {
    Unauthorized(Principal),
    NoSuchCredentials(String),
    InternalCanisterError(String),
}
