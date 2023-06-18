use hecs::World;
use rltk::{GameState, Rltk};

use crate::{resource::Map, system};

pub struct State {
    ecs: World,
    map: Map,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let ecs = &mut self.ecs;
        let map = &mut self.map;
        system::control_player(ctx, ecs, map);

        system::compute_visibilty(ctx, ecs, map);

        system::draw_map(ctx, ecs, map);
        system::render(ctx, ecs, map);
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            ecs: World::new(),
            map: Map::new(),
        }
    }

    pub fn ecs_mut(&mut self) -> &mut World {
        &mut self.ecs
    }

    pub fn ecs(&self) -> &World {
        &self.ecs
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut Map {
        &mut self.map
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State")
    }
}
