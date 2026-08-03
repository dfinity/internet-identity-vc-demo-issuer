//! Helpers for writing the issuer integration tests: PocketIC setup, loading
//! Wasm modules and a handful of fixtures.
//!
//! Vendored from `dfinity/internet-identity`,
//! `src/canister_tests/src/framework.rs`. Paths are relative to this
//! repository's root (where `cargo test` runs from) rather than to
//! `demos/vc_issuer` inside the II tree.

use crate::types::DeviceData;
use candid::Principal;
use lazy_static::lazy_static;
use pocket_ic::common::rest::{ExtendedSubnetConfigSet, SubnetSpec};
use pocket_ic::{PocketIc, PocketIcBuilder};
use serde_bytes::ByteBuf;
use std::env;
use std::path;
use url::Url;

/// Name of the environment variable the `pocket-ic` crate reads to locate the
/// server binary. CI sets it via the `dfinity/pocketic` action.
const POCKET_IC_BIN_ENV: &str = "POCKET_IC_BIN";

/// Where `scripts/fetch-pocket-ic` puts the pinned PocketIC server binary. A
/// binary at this path takes precedence over `POCKET_IC_BIN`, so a stale server
/// left on a developer's machine can't shadow the pinned version — a version
/// mismatch fails every test with a message that points at the wrong thing.
const POCKET_IC_PINNED_PATH: &str = "./pocket-ic";

lazy_static! {
    /// The gzipped Wasm module of the Internet Identity canister the issuer is
    /// tested against. Downloaded from `dfinity/internet-identity`'s releases
    /// (see `scripts/fetch-ii-wasm`).
    pub static ref II_WASM: Vec<u8> = {
        let def_path = path::PathBuf::from("./internet_identity.wasm.gz");
        let err = format!("
        Could not find the Internet Identity Wasm module.

        I will look for it at {:?}, and you can specify another path with the environment variable II_WASM (note that I run from {:?}).

        To download the module from Internet Identity's latest release, run:
            ./scripts/fetch-ii-wasm
        ", &def_path, &std::env::current_dir().map(|x| x.display().to_string()).unwrap_or_else(|_| "an unknown directory".to_string()));
        get_wasm_path("II_WASM".to_string(), &def_path).expect(&err)
    };
}

/// Returns the content of `default_path` if found, or `None` if the file does
/// not exist. The `env_var` environment variable is also read for a custom
/// location; if the variable is set _but_ the Wasm module is not present, we
/// panic (i.e. we don't return `None`).
pub fn get_wasm_path(env_var: String, default_path: &path::PathBuf) -> Option<Vec<u8>> {
    match env::var_os(env_var.clone()) {
        None => {
            if !default_path.exists() {
                return None;
            }
            Some(
                std::fs::read(default_path)
                    .unwrap_or_else(|_| panic!("could not read Wasm module: {default_path:?}")),
            )
        }
        Some(path) => {
            let pathname: String = path
                .into_string()
                .unwrap_or_else(|_| panic!("Invalid string path for {env_var}"));
            let path = path::PathBuf::from(pathname.clone());
            if !path.exists() {
                panic!("Could not find {pathname}");
            }
            Some(
                std::fs::read(path.clone())
                    .unwrap_or_else(|_| panic!("could not read Wasm module: {path:?}")),
            )
        }
    }
}

/// Builds a PocketIC instance with the subnet layout the issuer tests need.
pub fn env() -> PocketIc {
    let config = ExtendedSubnetConfigSet {
        ii: Some(SubnetSpec::default()),
        nns: Some(SubnetSpec::default()),
        ..ExtendedSubnetConfigSet::default()
    };

    // If there is a file named pocket-ic-port, reuse the PocketIC instance
    // listening on that port. This is necessary when the tests are driven by
    // nextest, which does not work with PocketIC being started by the tests
    // themselves.
    let port_file = path::PathBuf::from("./pocket-ic-port");
    if port_file.exists() {
        let port = std::fs::read_to_string(port_file).expect("failed to read port file");
        return PocketIcBuilder::new_with_config(config)
            .with_server_url(
                Url::try_from(format!("http://127.0.0.1:{port}").as_str()).expect("Invalid URL"),
            )
            .build();
    }

    let pinned_path = path::PathBuf::from(POCKET_IC_PINNED_PATH);
    if pinned_path.exists() {
        env::set_var(POCKET_IC_BIN_ENV, POCKET_IC_PINNED_PATH);
    } else if env::var_os(POCKET_IC_BIN_ENV).is_none() {
        panic!("
        Could not find the PocketIC server binary needed to run the integration tests. It is expected to be here: {:?},
        or at the path given by the {POCKET_IC_BIN_ENV} environment variable.
        Run ./scripts/fetch-pocket-ic to download the version pinned in .pocket-ic-version.",
        &pinned_path);
    }

    PocketIcBuilder::new_with_config(config).build()
}

pub const PUBKEY_1: &str = "test";

pub fn principal_1() -> Principal {
    Principal::self_authenticating(PUBKEY_1)
}

pub fn device_data_1() -> DeviceData {
    DeviceData {
        pubkey: ByteBuf::from(PUBKEY_1),
        alias: "My Device".to_string(),
        credential_id: Some(ByteBuf::from("credential id 1")),
        origin: Some("https://identity.internetcomputer.org".to_string()),
        ..DeviceData::auth_test_device()
    }
}

/// Adapted from `PrincipalId::new_user_test_id`.
pub fn test_principal(n: u64) -> Principal {
    let mut bytes = n.to_le_bytes().to_vec();
    bytes.push(0xfe); // internal marker for user test ids
    Principal::from_slice(&bytes[..])
}

pub fn time(env: &PocketIc) -> u64 {
    env.get_time().as_nanos_since_unix_epoch()
}
