# e2p-sys: Low-level Rust bindings for libe2p from e2fsprogs

[![CI](https://github.com/michaellass/e2p-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/michaellass/e2p-sys/actions/workflows/ci.yml)
[![license](https://img.shields.io/github/license/michaellass/e2p-sys.svg)](https://github.com/michaellass/e2p-sys/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/e2p-sys.svg)](https://crates.io/crates/e2p-sys)
[![docs.rs](https://docs.rs/e2p-sys/badge.svg)](https://docs.rs/e2p-sys)

This crate makes available functionality from libe2p. The bindings are automatically created by bindgen.

## Requirements
* libe2p, including development headers. Linux distributions typically packages those under one of the following names:
  * e2fsprogs(-dev)
  * e2fslibs(-dev)
  * libext2fs(-dev)
* libclang is required by bindgen. This is sometimes packaged as libclang1.

Note that after an update of libe2p, you may need to rebuild this crate to get access to any newly introduced flags.

## Intended Use
This crate should not be used directly. It's intended to be used to form higher-level abstractions.
