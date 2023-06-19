use rltk::Point;

#[derive(Debug, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: Vec::new(),
            range,
            dirty: true,
        }
    }

    pub fn is_visible(&self, point: &Point) -> bool {
        self.visible_tiles.contains(point)
    }
}
