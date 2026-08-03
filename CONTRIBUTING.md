# Contributing to the Internet Identity VC Demo Issuer

Thank you for your interest in contributing.

This repository is an example issuer for the [attribute sharing flow on the IC].
It demonstrates the issuer APIs and is used by [Internet Identity] for local
development and testing. It is **not meant for real-world deployment** — see the
warning in [README.md](README.md).

## Contributing code changes

If you want to contribute a feature or bug fix, please **reach out to us first**
so that we can discuss feasibility and implementation strategies. You can reach
out to us:

- on the [forum], or
- during our monthly [community calls].

Make sure to read the [LICENSE] first. Build, test and deployment instructions
are in [README.md](README.md).

Two things to keep in mind when proposing a change:

- Internet Identity consumes the release assets of a pinned tag rather than
  building this canister from source. The asset names are part of that contract —
  see the Releasing section of the README.
- `crates/asset_util` and `crates/ii_test_support` are copies of code that lives
  in Internet Identity, and the types in `crates/ii_test_support/src/types.rs`
  are a wire contract shared with it. Keep both sides in sync.

## Bug reports

We really appreciate bug reports through [GitHub tickets].

[GitHub tickets]: https://github.com/dfinity/internet-identity-vc-demo-issuer/issues/new
[Internet Identity]: https://github.com/dfinity/internet-identity
[attribute sharing flow on the IC]: https://github.com/dfinity/wg-identity-authentication/blob/main/topics/attribute-sharing.md
[forum]: https://forum.dfinity.org/c/internet-identity/32
[community calls]: https://forum.dfinity.org/t/working-group-identity-authentication/11902
[LICENSE]: LICENSE
