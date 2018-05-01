# in-situ

**In Situ Endian-independent Bytes Access**

[![Build Status][]](https://travis-ci.org/qu1x/in-situ)
[![Downloads][]](https://crates.io/crates/in-situ)
[![Version][]](https://crates.io/crates/in-situ)
[![Documentation][]](https://docs.rs/in-situ)
[![License][]](https://opensource.org/licenses/Fair)

[Build Status]: https://travis-ci.org/qu1x/in-situ.svg
[Downloads]: https://img.shields.io/crates/d/in-situ.svg
[Version]: https://img.shields.io/crates/v/in-situ.svg
[Documentation]: https://docs.rs/in-situ/badge.svg
[License]: https://img.shields.io/crates/l/in-situ.svg

## Contents

  * [Usage](#usage)
  * [License](#license)
  * [Contribution](#contribution)

## Usage

This crate is [on crates.io](https://crates.io/crates/in-situ) and can be
used by adding `in-situ` to the dependencies in your project's
`Cargo.toml`:

```toml
[dependencies]
in-situ = "0.1"

# Optionally enable `i128_type` support on nightly Rust.
#[dependencies.in-situ]
#features = ["i128"]
```

and this to your crate root:

```rust
// Optionally enable `i128_type` support on nightly Rust.
// Required if the `i128` feature is enabled in your `Cargo.toml`.
//#![feature(i128_type)]

extern crate in_situ;
```

## License

Copyright (c) 2018 Rouven Spreckels <n3vu0r@qu1x.org>

Usage of the works is permitted provided that
this instrument is retained with the works, so that
any entity that uses the works is notified of this instrument.

DISCLAIMER: THE WORKS ARE WITHOUT WARRANTY.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the works by you shall be licensed as above, without any
additional terms or conditions.
