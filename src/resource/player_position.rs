use rltk::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PlayerPosition(pub Point);
