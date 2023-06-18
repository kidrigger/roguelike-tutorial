use hecs::World;
use rltk::{Rltk, VirtualKeyCode};

use crate::{
    component::{Player, Position, Viewshed},
    resource::{Map, TileType},
};

pub fn control_player(ctx: &mut Rltk, ecs: &mut World, map: &Map) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Left
            | VirtualKeyCode::A
            | VirtualKeyCode::Numpad4
            | VirtualKeyCode::H => move_player((-1, 0), ecs, map),
            VirtualKeyCode::Right
            | VirtualKeyCode::D
            | VirtualKeyCode::Numpad6
            | VirtualKeyCode::L => move_player((1, 0), ecs, map),
            VirtualKeyCode::Up
            | VirtualKeyCode::W
            | VirtualKeyCode::Numpad8
            | VirtualKeyCode::K => move_player((0, -1), ecs, map),
            VirtualKeyCode::Down
            | VirtualKeyCode::S
            | VirtualKeyCode::Numpad2
            | VirtualKeyCode::J => move_player((0, 1), ecs, map),
            _ => (),
        }
    }
}

fn move_player((delta_x, delta_y): (i32, i32), ecs: &mut World, map: &Map) {
    for (_id, (_player, pos, viewshed)) in ecs
        .query::<(&Player, &mut Position, &mut Viewshed)>()
        .iter()
    {
        let next_x = pos.0.x + delta_x;
        let next_y = pos.0.y + delta_y;
        if map.get_tile_type(next_x, next_y) != TileType::Wall {
            pos.0.x = next_x.clamp(0, Map::WIDTH);
            pos.0.y = next_y.clamp(0, Map::HEIGHT);
        }

        viewshed.dirty = true;
    }
}
