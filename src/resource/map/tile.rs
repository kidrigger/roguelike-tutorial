use rltk::{BLACK, RGB};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum TileType {
    #[default]
    Floor,
    Wall,
}

impl TileType {
    const FLOOR_COLOR: RGB = RGB {
        r: 0.0,
        g: 0.6,
        b: 0.4,
    };

    const WALL_COLOR: RGB = RGB {
        r: 0.0,
        g: 0.8,
        b: 0.0,
    };

    pub fn glyph(&self) -> u16 {
        let gly = match self {
            TileType::Floor => '.',
            TileType::Wall => '#',
        };

        rltk::to_cp437(gly)
    }

    pub fn foreground(&self) -> RGB {
        match self {
            TileType::Floor => Self::FLOOR_COLOR,
            TileType::Wall => Self::WALL_COLOR,
        }
    }

    pub fn background(&self) -> RGB {
        RGB::from(BLACK)
    }
}
