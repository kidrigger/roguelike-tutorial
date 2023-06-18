use hecs::World;
use rltk::{Rltk, VirtualKeyCode};

use crate::{
    component::{Player, Position},
    resource::{Map, TileType},
};

pub fn control_player(ctx: &mut Rltk, ecs: &mut World) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Left => move_player((-1, 0), ecs),
            VirtualKeyCode::Right => move_player((1, 0), ecs),
            VirtualKeyCode::Up => move_player((0, -1), ecs),
            VirtualKeyCode::Down => move_player((0, 1), ecs),
            _ => (),
        }
    }
}

fn move_player((delta_x, delta_y): (i32, i32), ecs: &mut World) {
    let mut q = ecs.query::<&Map>();
    let (_map_id, map) = q.iter().next().unwrap();

    for (_id, (_player, pos)) in ecs.query::<(&Player, &mut Position)>().iter() {
        let next_x = pos.x + delta_x;
        let next_y = pos.y + delta_y;
        if map.get_tile_type(next_x, next_y) != TileType::Wall {
            pos.x = next_x.clamp(0, Map::WIDTH);
            pos.y = next_y.clamp(0, Map::HEIGHT);
        }
    }
}
