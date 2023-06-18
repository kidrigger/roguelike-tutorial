use hecs::World;
use rltk::Rltk;

use crate::{
    component::{Position, Renderable},
    resource::Map,
};

pub fn render(ctx: &mut Rltk, ecs: &mut World, map: &Map) {
    for (_id, (pos, render)) in ecs.query_mut::<(&Position, &Renderable)>() {
        if map.is_visible(pos.0.x, pos.0.y) {
            ctx.set(
                pos.0.x,
                pos.0.y,
                render.foreground,
                render.background,
                render.glyph,
            );
        }
    }
}
