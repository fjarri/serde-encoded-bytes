# Efficient bytestring serialization for `serde`-supporting types

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![License][license-image]
[![Build Status][build-image]][build-link]
[![Coverage][coverage-image]][coverage-link]

## What it does

Byte arrays (`[u8; N]`, `Vec<u8>`, `Box<[u8]>` and so on) are treated by `serde` as arrays of integers, which leads to inefficient representations in various formats.
For example, this is how serialization works by default for binary and human-readable formats:
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Array([u8; 16]);

let array = Array([
    0, 1, 0xf2, 3, 0xf4, 5, 0xf6, 7,
    0xf8, 9, 0xfa, 11, 0xfc, 13, 14, 0xff
]);

// Serializing as MessagePack
assert_eq!(
    rmp_serde::encode::to_vec(&array).unwrap(),
    [
        220, 0, 16, 0, 1, 204, 242, 3, 204, 244, 5, 204, 246, 7,
        204, 248, 9, 204, 250, 11, 204, 252, 13, 14, 204, 255
    ]
);

// Serializing as JSON
assert_eq!(
    serde_json::to_string(&array).unwrap(),
    "[0,1,242,3,244,5,246,7,248,9,250,11,252,13,14,255]"
);
```
Note that in MessagePack the bytes with the value above `0x7f` (that is, the ones with the MSB set) are prefixed by `0xcc` (`204`), which makes the bytestring take more space than it should.
And in case of JSON, the bytestring is serialized as an array of integers, which is also not very efficient.

This crate provides methods that can be used in [`serde(with)`](https://serde.rs/field-attrs.html#with) field attribute to amend this behavior and serialize bytestrings efficiently, verbatim in binary formats, or using the selected encoding in human-readable formats.


## Usage

To use, add a [`serde(with)`](https://serde.rs/field-attrs.html#with) annotation with an argument composed of a container type (whether it is array-like, slice-like and so on) and the desired encoding:
```rust
use serde::{Deserialize, Serialize};
use serde_encoded_bytes::{ArrayLike, Hex};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Array(#[serde(with = "ArrayLike::<Hex>")] [u8; 16]);

let array = Array([
    0, 1, 0xf2, 3, 0xf4, 5, 0xf6, 7,
    0xf8, 9, 0xfa, 11, 0xfc, 13, 14, 0xff,
]);

// Serializing as MessagePack
assert_eq!(
    rmp_serde::encode::to_vec(&array).unwrap(),
    [196, 16, 0, 1, 242, 3, 244, 5, 246, 7, 248, 9, 250, 11, 252, 13, 14, 255]
);

// Serializing as JSON
assert_eq!(
    serde_json::to_string(&array).unwrap(),
    "\"0x0001f203f405f607f809fa0bfc0d0eff\""
);
```
As you can see, the serialization of the example above is now more efficient in either format.

Note that due to `serde` limitations (see <https://github.com/serde-rs/serde/issues/2120>) fixed-size arrays will still be serialized with their length included in binary formats.


## Tested formats

While this crate is supposed to work for any format that supports `serde`, it is specifically tested on:
- [`bincode`](https://crates.io/crates/bincode) v2.0.0-rc.3
- [`ciborium`](https://crates.io/crates/ciborium) v0.2
- [`rmp-serde`](https://crates.io/crates/rmp-serde) v1
- [`serde-json-core`](https://crates.io/crates/serde-json-core) v0.6
- [`serde-json`](https://crates.io/crates/serde-json) v1
- [`toml`](https://crates.io/crates/toml) v0.8


## Prior art

More established crates with intersecting functionality include:
- [`serde-bytes-repr`](https://crates.io/crates/serde-bytes-repr) - similar capabilities, but a different approach to the API, which can be inconvenient in a number of cases;
- [`serde_bytes`](https://crates.io/crates/serde_bytes) - provides efficient serialization in binary formats, but not in human-readable formats;
- [`serdect`](https://crates.io/crates/serdect) - focused specifically on constant-time serialization;
- [`hex-buffer-serde`](https://crates.io/crates/hex-buffer-serde) - fixed encoding;
- [`base64-serde`](https://crates.io/crates/base64-serde), [`serde-hex`](https://crates.io/crates/serde-hex), [`stremio-serde-hex`](https://crates.io/crates/stremio-serde-hex) - fixed encoding, and no support for binary formats.



[crate-image]: https://img.shields.io/crates/v/serde-encoded-bytes.svg
[crate-link]: https://crates.io/crates/serde-encoded-bytes
[docs-image]: https://docs.rs/serde-encoded-bytes/badge.svg
[docs-link]: https://docs.rs/serde-encoded-bytes/
[license-image]: https://img.shields.io/crates/l/serde-encoded-bytes
[build-image]: https://github.com/fjarri/serde-encoded-bytes/actions/workflows/ci.yml/badge.svg?branch=master&event=push
[build-link]: https://github.com/fjarri/serde-encoded-bytes/actions?query=workflow%3Aci
[coverage-image]: https://codecov.io/gh/fjarri/serde-encoded-bytes/branch/master/graph/badge.svg
[coverage-link]: https://codecov.io/gh/fjarri/serde-encoded-bytes
