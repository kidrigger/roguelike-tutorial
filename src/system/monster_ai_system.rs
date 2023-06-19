use hecs::World;
use rltk::{console, Rltk};

use crate::{
    component::{Monster, Position, Viewshed},
    resource::{PlayerPosition, Resources},
};

pub fn compute_monster_behavior(_ctx: &mut Rltk, ecs: &mut World, res: &mut Resources) {
    type MonsterQueryType<'a> = (&'a Viewshed, &'a Position, &'a Monster);
    let player_pos = res.fetch::<PlayerPosition>().0;

    for (_id, (view, _pos, _monster)) in ecs.query_mut::<MonsterQueryType>() {
        if view.is_visible(&player_pos) {
            console::log("Monster shouts insultes");
        }
    }
}
