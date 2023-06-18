use rltk::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Position(pub Point);

impl Position {
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: Into<i32>,
    {
        Self(Point::new(x, y))
    }
}

impl<T> From<(T, T)> for Position
where
    T: Into<i32>,
{
    fn from((x, y): (T, T)) -> Self {
        Self(Point::new(x, y))
    }
}

impl From<Point> for Position {
    fn from(value: Point) -> Self {
        Self(value)
    }
}
