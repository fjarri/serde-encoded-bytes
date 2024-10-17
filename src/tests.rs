//! Integration tests to check compatibility with popular serializer libraries.

#![allow(clippy::unwrap_used)]

mod asn1;
mod bincode;
mod cbor;
mod common;
mod messagepack;
mod serde_json;
mod serde_json_core;
mod toml;
