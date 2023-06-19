use hecs::World;
use rltk::Rltk;

use crate::{
    component::{Position, TileBlock},
    resource::{Map, Resources},
};

pub fn index_map(_ctx: &mut Rltk, ecs: &mut World, res: &mut Resources) {
    type BlockingQueryType<'a> = (&'a Position, &'a TileBlock);

    let map = res.fetch_mut::<Map>();

    map.populate_blocked();
    map.clear_content_index();

    for (id, (pos, _block)) in ecs.query_mut::<BlockingQueryType>() {
        map.set_tile_blocked(&pos.0, true);

        map.push_tile_content(&pos.0, id);
    }
}
