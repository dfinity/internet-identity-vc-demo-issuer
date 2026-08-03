# An example issuer of verifiable credentials

This is an example implementation of an issuer of verifiable credentials for
the [attribute sharing flow on the IC](https://github.com/dfinity/wg-identity-authentication/blob/main/topics/attribute-sharing.md).

## Supported Behavior

The app implements the interface of the issuer from
[the spec](https://github.com/dfinity/internet-identity/blob/main/docs/vc-spec.md),
plus some additional APIs for configuring and testing, cf.
[vc_demo_issuer.did](./vc_demo_issuer.did). These additional APIs enable
"self-registration" of users, so that the issuer can issue credentials for them.
The issuer also offers a simple FE component, that allows for
"self-registration" via a browser.

**WARNING:** Please note that this issuer is for demonstrating the use of issuer APIs, and is **not meant
for real-world deployment**, as it does not have proper management of user data.

## Layout

| Path                     | Contents                                                                              |
| ------------------------ | ------------------------------------------------------------------------------------- |
| `src/`                   | The issuer canister (Rust).                                                           |
| `app/`                   | The issuer's self-registration frontend (React), embedded into the canister.          |
| `vc_demo_issuer.did`     | The canister's Candid interface. Internet Identity generates bindings from this file. |
| `provision`              | Configures a deployed issuer canister for a given environment.                        |
| `crates/asset_util`      | Asset certification helpers, vendored from Internet Identity.                         |
| `crates/ii_test_support` | Internet Identity wire types and PocketIC helpers, used by the tests only.            |

`crates/asset_util` and `crates/ii_test_support` are copies of code that lives in
[dfinity/internet-identity](https://github.com/dfinity/internet-identity). The
types in `crates/ii_test_support/src/types.rs` are a wire contract shared with
Internet Identity — if II changes them, they must be updated here too.

## Building

Requires the Rust toolchain from `rust-toolchain.toml`, the Node version from
`.node-version`, and `ic-wasm`. `./scripts/bootstrap` installs the toolchain and
`ic-wasm`.

```bash
npm ci
./build.sh
```

This produces `vc_demo_issuer.wasm.gz`.

Regenerating the frontend's Candid bindings after editing `vc_demo_issuer.did`
requires the `didc` release pinned in `.didc-release`:

```bash
npm run generate
```

## Testing

The integration tests in `tests/issue_credential.rs` run the issuer against the
**latest Internet Identity release**, so they need three artifacts in the
repository root:

```bash
./build.sh                  # vc_demo_issuer.wasm.gz — the canister under test
./scripts/fetch-ii-wasm     # internet_identity.wasm.gz — from II's latest release
./scripts/fetch-pocket-ic   # pocket-ic — the server version pinned in .pocket-ic-version
cargo test
```

The PocketIC **server** version has to match the `pocket-ic` crate version in
`Cargo.toml`; a mismatch fails every test with a misleading error. That is why it
is pinned in `.pocket-ic-version`, read by both `scripts/fetch-pocket-ic` and CI,
and why a `./pocket-ic` in the repository root takes precedence over any
`POCKET_IC_BIN` already set in the environment.

Because the tests track II's `latest` release rather than a pinned tag, a new II
release can break this repository on its own. CI therefore runs the tests on a
weekly schedule as well as on every pull request.

## Releasing

Internet Identity consumes this repository's **GitHub Releases**: it pins a tag
in its `.github/versions/vc-issuer` file and downloads the release assets, so it
never builds the issuer from source.

Push a `release-*` tag to publish a release:

```bash
git tag release-1.0.0
git push origin release-1.0.0
```

`.github/workflows/release.yml` builds the canister and publishes these assets.
The names are part of the contract with Internet Identity and must not change:

| Asset                           | Consumed by                                            |
| ------------------------------- | ------------------------------------------------------ |
| `vc_demo_issuer.wasm.gz`        | II's mainnet deploy job and local `icp deploy issuer`. |
| `vc_demo_issuer.wasm.gz.sha256` | Integrity check in II's `scripts/fetch-vc-issuer`.     |
| `vc_demo_issuer.did`            | II's `didc bind` codegen for the issuer's JS/TS types. |
| `provision`                     | Executed by II's deploy job after installing the wasm. |

After a release, II's scheduled bump workflow opens a pull request moving
`.github/versions/vc-issuer` to the new tag, refreshing II's vendored copy of
`vc_demo_issuer.did` and regenerating the bindings.

## Deploying locally

To use the demo issuer in a testing setup, the issuer canister has to be
configured to match the environment, i.e. it has to use the correct root public
key, and accept requests from the correct II canister — see the
[`provision` script](./provision) for details. `icp.yaml.example` shows a minimal
icp-cli project that builds and provisions the issuer alongside a remote II.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Security issues are handled through the
process in [SECURITY.md](SECURITY.md) — please do not report them as public
issues.

## License

This project is licensed under the Internet Computer Community Source License
v1.0, the same license as [Internet Identity](https://github.com/dfinity/internet-identity).
See [LICENSE](LICENSE).
