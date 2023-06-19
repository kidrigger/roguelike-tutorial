use hecs::World;
use rltk::{Point, Rltk};

use crate::resource::{Map, Resources};

pub fn draw_map(ctx: &mut Rltk, _ecs: &mut World, res: &mut Resources) {
    let map = res.fetch::<Map>();
    for (index, tile) in map.tiles().iter().enumerate() {
        let Point { x, y } = map.index_to_point(index);
        if map.is_visible(x, y) {
            ctx.set(x, y, tile.foreground(), tile.background(), tile.glyph());
        } else if map.is_revealed(x, y) {
            ctx.set(
                x,
                y,
                tile.foreground().to_greyscale(),
                tile.background(),
                tile.glyph(),
            );
        }
    }
}
