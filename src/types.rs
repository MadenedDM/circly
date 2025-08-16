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
    use std::ops::{Add, Sub};

    use rkyv::{Archive, Deserialize, Serialize};

    #[derive(Clone, Archive, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
    #[rkyv(compare(PartialEq), derive(Debug))]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Self::Output {
                x: self.x.saturating_add(rhs.x),
                y: self.y.saturating_add(rhs.y),
            }
        }
    }

    impl Sub for Point {
        type Output = Point;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::Output {
                x: self.x.saturating_sub(rhs.x),
                y: self.y.saturating_sub(rhs.y),
            }
        }
    }
}
