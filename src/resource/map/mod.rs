use rltk::Rltk;

use self::tile::TileType;

pub mod tile;

#[derive(Debug, Clone)]
pub struct Map(Vec<tile::TileType>);

impl Map {
    pub const WIDTH: i32 = 80;
    pub const HEIGHT: i32 = 50;

    pub fn new() -> Self {
        let mut map = vec![TileType::Floor; Self::WIDTH as usize * Self::HEIGHT as usize];

        for x in 0..Self::WIDTH {
            map[Self::index(x, 0)] = TileType::Wall;
            map[Self::index(x, Self::HEIGHT - 1)] = TileType::Wall;
        }

        for y in 0..Self::HEIGHT {
            map[Self::index(0, y)] = TileType::Wall;
            map[Self::index(Self::WIDTH - 1, y)] = TileType::Wall;
        }

        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..400 {
            let x = rng.roll_dice(1, Self::WIDTH - 1);
            let y = rng.roll_dice(1, Self::HEIGHT - 1);

            let idx = Self::index(x, y);

            if idx != Self::index(40, 25) {
                map[idx] = TileType::Wall;
            }
        }

        Self(map)
    }

    fn index(x: i32, y: i32) -> usize {
        (y as usize * Self::WIDTH as usize) + x as usize
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        (0..Self::HEIGHT)
            .flat_map(|j| (0..Self::WIDTH).map(move |i| (i, j)))
            .zip(self.0.iter())
            .for_each(|((x, y), tile)| {
                ctx.set(x, y, tile.foreground(), tile.background(), tile.glyph())
            });
    }

    pub fn get_tile_type(&self, x: i32, y: i32) -> TileType {
        self.0[Self::index(x, y)]
    }
}
