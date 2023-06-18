use hecs::World;
use rltk::Rltk;

use crate::component::{Position, Renderable};

pub fn render(ctx: &mut Rltk, ecs: &mut World) {
    for (_id, (pos, render)) in ecs.query_mut::<(&Position, &Renderable)>() {
        ctx.set(
            pos.x,
            pos.y,
            render.foreground,
            render.background,
            render.glyph,
        );
    }
}
