# in-situ

[![Build][]](https://github.com/qu1x/in-situ/actions/workflows/build.yml)
[![Documentation][]](https://docs.rs/in-situ)
[![Downloads][]](https://crates.io/crates/in-situ)
[![Version][]](https://crates.io/crates/in-situ)
[![Rust][]](https://www.rust-lang.org)
[![License][]](https://opensource.org/licenses)

[Build]: https://github.com/qu1x/in-situ/actions/workflows/build.yml/badge.svg
[Documentation]: https://docs.rs/in-situ/badge.svg
[Downloads]: https://img.shields.io/crates/d/in-situ.svg
[Version]: https://img.shields.io/crates/v/in-situ.svg
[Rust]: https://img.shields.io/badge/rust-v1.85.0-brightgreen.svg
[License]: https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg

In Situ Endian-iIndependent Bytes Access

## Feature Gates

  * `bytes`: For abstracting `Bytes` and `BytesMut` as well.
  * `bstr`: For complementing `InSitu::utf8()` with `InSitu::bstr()`.

## License

This work is dual-licensed under either [`MIT`] or [`Apache-2.0`] at your option.

[`MIT`]: LICENSE-MIT
[`Apache-2.0`]: LICENSE-APACHE

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the [`Apache-2.0`] license, shall be dual-licensed as above, without any
additional terms or conditions.
