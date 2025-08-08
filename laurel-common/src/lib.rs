pub mod color {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
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

pub mod lore {
    
}