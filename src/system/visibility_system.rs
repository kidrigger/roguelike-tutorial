use hecs::World;
use rltk::{field_of_view, Rltk};

use crate::{
    component::{Player, Position, Viewshed},
    resource::Map,
};

pub fn compute_visibilty(_ctx: &mut Rltk, ecs: &mut World, map: &mut Map) {
    for (_id, (pos, mut viewshed)) in ecs.query::<(&Position, &mut Viewshed)>().iter() {
        if viewshed.dirty {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(pos.0, viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| (0..Map::WIDTH).contains(&p.x) && (0..Map::HEIGHT).contains(&p.y));

            if ecs
                .query_one::<&Player>(_id)
                .map(|mut v| v.get().is_some())
                .unwrap_or(false)
            {
                for t in map.visible_tiles_mut().iter_mut() {
                    *t = false
                }
                for vis in viewshed.visible_tiles.iter() {
                    let idx = map.point_to_index(vis);
                    map.revealed_tiles_mut()[idx] = true;
                    map.visible_tiles_mut()[idx] = true;
                }
            }
        }
    }
}
