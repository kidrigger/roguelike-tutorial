use hecs::World;
use rltk::{GameState, Rltk};

use crate::system;

pub struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let ecs = &mut self.ecs;
        system::control_player(ctx, ecs);

        system::draw_map(ctx, ecs);
        system::render(ctx, ecs);
    }
}

impl State {
    pub fn new() -> Self {
        Self { ecs: World::new() }
    }

    pub fn ecs_mut(&mut self) -> &mut World {
        &mut self.ecs
    }

    pub fn ecs(&self) -> &World {
        &self.ecs
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State")
    }
}
