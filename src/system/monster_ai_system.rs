use hecs::World;
use rltk::{console, Rltk};

use crate::{
    component::{Monster, Name, Position, Viewshed},
    resource::{Map, PlayerPosition, Resources},
};

pub fn compute_monster_behavior(_ctx: &mut Rltk, ecs: &mut World, res: &mut Resources) {
    type MonsterQueryType<'a> = (&'a mut Viewshed, &'a Name, &'a mut Position, &'a Monster);
    let player_pos = res.fetch::<PlayerPosition>().0;

    let map = res.fetch::<Map>();

    for (_id, (view, name, pos, _monster)) in ecs.query_mut::<MonsterQueryType>() {
        if view.is_visible(&player_pos) {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(pos.0, player_pos);
            if distance < 1.5 {
                console::log(format!("{name} shouts insults"));
                continue;
            }

            let path = rltk::a_star_search(
                map.point_to_index(&pos.0),
                map.point_to_index(&player_pos),
                map,
            );
            if path.success && path.steps.len() > 1 {
                pos.0 = map.index_to_point(path.steps[1]);
                view.dirty = true;
            }
        }
    }
}
