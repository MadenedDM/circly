#![allow(clippy::multiple_crate_versions)]
pub mod camphor;
pub mod components;
pub mod types;

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/mainservice.rs"));
}
