#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}
