use hecs::World;
use rltk::{GameState, Rltk};

use crate::{
    resource::{Map, Resources},
    system,
};

pub struct State {
    ecs: World,
    res: Resources,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let ecs = &mut self.ecs;
        let res = &mut self.res;
        system::control_player(ctx, ecs, res);

        system::compute_visibilty(ctx, ecs, res);

        system::draw_map(ctx, ecs, res);
        system::render(ctx, ecs, res);
    }
}

impl State {
    pub fn new() -> Self {
        let ecs = World::new();
        let mut res = Resources::new();
        res.insert(Map::new());

        Self { ecs, res }
    }

    pub fn ecs_mut(&mut self) -> &mut World {
        &mut self.ecs
    }

    pub fn ecs(&self) -> &World {
        &self.ecs
    }

    pub fn res(&self) -> &Resources {
        &self.res
    }

    pub fn res_mut(&mut self) -> &mut Resources {
        &mut self.res
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State")
    }
}
