//! Test-only support code for the integration tests in `tests/`.
//!
//! Everything here is vendored from `dfinity/internet-identity`:
//!  * `types`: the II ↔ issuer wire types (`src/internet_identity_interface`).
//!  * `api`: Rust bindings for the II canister methods the tests call
//!    (`src/canister_tests/src/api`).
//!  * `flows`: reusable flows made of several canister interactions
//!    (`src/canister_tests/src/flows.rs`).
//!  * `framework`: PocketIC setup and Wasm-loading helpers
//!    (`src/canister_tests/src/framework.rs`).
//!
//! Only the subset the issuer tests actually use is vendored. The types in
//! `types` are a wire contract shared with II — they must stay byte-compatible
//! with II's definitions or the tests will decode garbage.

pub mod api;
pub mod flows;
pub mod framework;
pub mod types;
