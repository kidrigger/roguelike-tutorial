use hecs::World;
use rltk::Rltk;

use crate::resource::Map;

pub fn draw_map(ctx: &mut Rltk, ecs: &mut World) {
    for (_id, map) in ecs.query_mut::<&Map>() {
        map.draw(ctx);
    }
}
