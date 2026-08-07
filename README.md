tre-regex-sys
-------------

[![CI](https://github.com/Elizafox/tre-regex-sys/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Elizafox/tre-regex-sys/actions/workflows/ci.yml)

This is a crate that builds [bindgen](https://crates.io/crates/bindgen) bindings for [tre](https://laurikari.net/tre/).

This does **NOT** provide a safe API wrapper! See [tre-regex](https://crates.io/crates/tre-regex) for a safe API wrapper.

For documentation, see the [docs](https://docs.rs/tre-regex-sys) and the [official TRE documentation](https://laurikari.net/tre/documentation/).

Feature flags
=============

The following features are available:

- `approx`: Enable approximate matching functionality (enabled by default)
- `vendored`: Enable vendored build copy of TRE instead of system TRE (enabled by default)
- `wchar`: Enable functions designed to work with `wchar_t` (disabled by default)

Supported versions
==================

The vendored build uses TRE 0.9.0. System builds support TRE 0.8.0 or newer,
although TRE 0.9.0 is recommended for its additional byte-oriented APIs and
bug fixes.

Contributing
============

Contributions are welcome. Please follow the [Code of Conduct](CODE_OF_CONDUCT.md)
and guidelines set out by the [Developer Certificate of Origin](DCO).
