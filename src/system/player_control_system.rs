use hecs::World;
use rltk::{Rltk, VirtualKeyCode};

use crate::{
    component::{CombatStats, Player, Position, Viewshed},
    resource::{Map, PlayerPosition, Resources, RunState, TileType},
};

pub fn control_player(ctx: &mut Rltk, ecs: &mut World, res: &mut Resources) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Left
            | VirtualKeyCode::A
            | VirtualKeyCode::Numpad4
            | VirtualKeyCode::H => move_player((-1, 0), ecs, res),
            VirtualKeyCode::Right
            | VirtualKeyCode::D
            | VirtualKeyCode::Numpad6
            | VirtualKeyCode::L => move_player((1, 0), ecs, res),
            VirtualKeyCode::Up
            | VirtualKeyCode::W
            | VirtualKeyCode::Numpad8
            | VirtualKeyCode::K => move_player((0, -1), ecs, res),
            VirtualKeyCode::Down
            | VirtualKeyCode::S
            | VirtualKeyCode::Numpad2
            | VirtualKeyCode::J => move_player((0, 1), ecs, res),
            VirtualKeyCode::Q | VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => {
                move_player((-1, -1), ecs, res)
            }
            VirtualKeyCode::E | VirtualKeyCode::Numpad9 | VirtualKeyCode::U => {
                move_player((1, -1), ecs, res)
            }
            VirtualKeyCode::Z | VirtualKeyCode::Numpad1 | VirtualKeyCode::B => {
                move_player((-1, 1), ecs, res)
            }
            VirtualKeyCode::C | VirtualKeyCode::Numpad3 | VirtualKeyCode::N => {
                move_player((1, 1), ecs, res)
            }
            _ => (),
        }
    }
}

fn move_player((delta_x, delta_y): (i32, i32), ecs: &mut World, res: &mut Resources) {
    type PlayerQueryType<'a> = (
        &'a Player,
        &'a mut Position,
        &'a mut Viewshed,
        &'a CombatStats,
    );
    let map = res.fetch::<Map>();

    let mut player_pos = None;
    for (_id, (_player, pos, viewshed, _combat)) in ecs.query::<PlayerQueryType>().iter() {
        let next_x = pos.0.x + delta_x;
        let next_y = pos.0.y + delta_y;

        let next_idx = Map::xy_to_index(next_x, next_y);

        if !map.is_blocked(next_x, next_y) {
            pos.0.x = next_x.clamp(0, Map::WIDTH);
            pos.0.y = next_y.clamp(0, Map::HEIGHT);
            player_pos = Some(pos.0);
        }

        viewshed.dirty = true;
    }

    if let Some(pos) = player_pos {
        res.fetch_mut::<PlayerPosition>().0 = pos;
        res.fetch_mut::<RunState>().run();
    }
}
