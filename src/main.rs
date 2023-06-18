use roguelike::{
    component::{Player, Position, Renderable},
    resource::Map,
    state::State,
};

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State::new();
    gs.ecs_mut().spawn((
        Player,
        Position::new(40, 25),
        Renderable::new('☺', rltk::RED, None),
    ));
    gs.ecs_mut().spawn((Map::new_test(),));

    rltk::main_loop(context, gs)
}
