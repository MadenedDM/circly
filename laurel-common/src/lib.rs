#![allow(clippy::multiple_crate_versions)]

pub mod lore;

/// Checks version of the crate for comparison in case of breaking changes between server and client.
#[must_use]
pub fn version() -> u32 {
    env!("CARGO_PKG_VERSION_MAJOR")
        .parse::<u32>()
        .unwrap_or_default()
}

pub mod color {
    use rkyv::{Archive, Deserialize, Serialize};

    #[derive(Clone, Archive, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
    #[rkyv(compare(PartialEq), derive(Debug))]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Color {
        #[must_use]
        pub fn new(r: u8, g: u8, b: u8) -> Self {
            Self { r, g, b }
        }
    }
}

pub mod point {
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
}