use hecs::Entity;
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rect};

use self::tile::TileType;

pub mod tile;

#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<tile::TileType>,
    rooms: Vec<Rect>,
    revealed_tiles: Vec<bool>,
    visible_tiles: Vec<bool>,
    blocked: Vec<bool>,
    tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub const WIDTH: i32 = 80;
    pub const HEIGHT: i32 = 50;

    pub const MAX_ROOMS: i32 = 30;
    pub const MIN_SIZE: i32 = 6;
    pub const MAX_SIZE: i32 = 10;

    pub fn new() -> Self {
        const SIZE: usize = Map::WIDTH as usize * Map::HEIGHT as usize;
        let mut tiles = vec![TileType::Wall; SIZE];

        let mut rng = RandomNumberGenerator::new();

        let mut rooms = Vec::new();

        for _ in 0..Self::MAX_ROOMS {
            let w = rng.range(Self::MIN_SIZE, Self::MAX_SIZE);
            let h = rng.range(Self::MIN_SIZE, Self::MAX_SIZE);
            let x = rng.roll_dice(1, Self::WIDTH - w - 1) - 1;
            let y = rng.roll_dice(1, Self::HEIGHT - h - 1) - 1;

            let new_room = Rect {
                x1: x,
                x2: x + w,
                y1: y,
                y2: y + h,
            };

            let ok = !rooms
                .iter()
                .any(|other_room| new_room.intersect(other_room));

            if ok {
                apply_room_to_map(&mut tiles, &new_room);

                if !rooms.is_empty() {
                    let Point { x: new_x, y: new_y } = new_room.center();
                    let Point {
                        x: prev_x,
                        y: prev_y,
                    } = rooms.last().unwrap().center();
                    if rng.rand::<bool>() {
                        apply_horizontal_tunnel(&mut tiles, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut tiles, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut tiles, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut tiles, prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }
        }

        let revealed_tiles = vec![false; SIZE];
        let visible_tiles = vec![false; SIZE];
        let blocked = vec![false; SIZE];
        let tile_content = vec![Vec::new(); SIZE];

        Self {
            tiles,
            rooms,
            revealed_tiles,
            visible_tiles,
            blocked,
            tile_content,
        }
    }

    pub fn xy_to_index(x: i32, y: i32) -> usize {
        (y as usize * Map::WIDTH as usize) + x as usize
    }

    pub fn point_to_index(&self, xy: &Point) -> usize {
        Self::xy_to_index(xy.x, xy.y)
    }

    pub fn index_to_point(&self, index: usize) -> Point {
        let x = index % Self::WIDTH as usize;
        let y = index / Self::WIDTH as usize;

        Point::new(x as i32, y as i32)
    }

    pub fn get_tile_type(&self, x: i32, y: i32) -> TileType {
        self.tiles[Self::xy_to_index(x, y)]
    }

    pub fn is_revealed(&self, x: i32, y: i32) -> bool {
        self.revealed_tiles[Self::xy_to_index(x, y)]
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        self.visible_tiles[Self::xy_to_index(x, y)]
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.blocked[Self::xy_to_index(x, y)]
    }

    pub fn tiles(&self) -> &[tile::TileType] {
        &self.tiles
    }

    pub fn rooms(&self) -> &[Rect] {
        &self.rooms
    }

    pub fn revealed_tiles_mut(&mut self) -> &mut Vec<bool> {
        &mut self.revealed_tiles
    }

    pub fn revealed_tiles(&self) -> &[bool] {
        self.revealed_tiles.as_ref()
    }

    pub fn visible_tiles(&self) -> &[bool] {
        self.visible_tiles.as_ref()
    }

    pub fn visible_tiles_mut(&mut self) -> &mut Vec<bool> {
        &mut self.visible_tiles
    }

    pub fn blocked(&self) -> &[bool] {
        self.blocked.as_ref()
    }

    pub fn blocked_mut(&mut self) -> &mut Vec<bool> {
        &mut self.blocked
    }

    pub fn tile_content(&self) -> &[Vec<Entity>] {
        self.tile_content.as_ref()
    }

    pub fn tile_content_mut(&mut self) -> &mut Vec<Vec<Entity>> {
        &mut self.tile_content
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocked[i] = tile.is_opaque();
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn set_tile_blocked(&mut self, point: &Point, is_blocked: bool) {
        let idx = self.point_to_index(point);
        self.blocked[idx] = is_blocked;
    }

    pub fn push_tile_content(&mut self, point: &Point, ent: Entity) {
        let idx = self.point_to_index(point);
        self.tile_content[idx].push(ent);
    }

    pub fn get_tile_content(&self, x: i32, y: i32) -> &[Entity] {
        &self.tile_content[Self::xy_to_index(x, y)]
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if !(1..Self::WIDTH).contains(&x) || !(1..Self::HEIGHT).contains(&y) {
            return false;
        }
        !self.is_blocked(x, y)
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(Self::WIDTH, Self::HEIGHT)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].is_opaque()
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();

        let Point { x, y } = self.index_to_point(idx);

        // Cardinal
        if self.is_exit_valid(x - 1, y) {
            exits.push((Self::xy_to_index(x - 1, y), 1.0))
        }
        if self.is_exit_valid(x + 1, y) {
            exits.push((Self::xy_to_index(x + 1, y), 1.0))
        }
        if self.is_exit_valid(x, y - 1) {
            exits.push((Self::xy_to_index(x, y - 1), 1.0))
        }
        if self.is_exit_valid(x, y + 1) {
            exits.push((Self::xy_to_index(x, y + 1), 1.0))
        }

        // Diagonal
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push((Self::xy_to_index(x - 1, y - 1), 1.45))
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push((Self::xy_to_index(x - 1, y + 1), 1.45))
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push((Self::xy_to_index(x + 1, y - 1), 1.45))
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push((Self::xy_to_index(x + 1, y + 1), 1.45))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = self.index_to_point(idx1);
        let p2 = self.index_to_point(idx2);

        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

// Private

fn apply_room_to_map(map: &mut [TileType], room: &Rect) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[Map::xy_to_index(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], px: i32, nx: i32, y: i32) {
    let x1 = px.min(nx);
    let x2 = px.max(nx);
    for x in x1..=x2 {
        map[Map::xy_to_index(x, y)] = TileType::Floor;
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], py: i32, ny: i32, x: i32) {
    let y1 = py.min(ny);
    let y2 = py.max(ny);
    for y in y1..=y2 {
        map[Map::xy_to_index(x, y)] = TileType::Floor;
    }
}
