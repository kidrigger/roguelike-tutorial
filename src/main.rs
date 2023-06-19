use rltk::Rect;
use roguelike::{
    component::{Monster, Player, Position, Renderable, Viewshed},
    resource::{Map, PlayerPosition},
    state::State,
};

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State::new();
    let player_pos = gs.res().fetch::<Map>().rooms().first().unwrap().center();
    gs.res_mut().insert(PlayerPosition(player_pos));

    let monster_positions = gs
        .res()
        .fetch::<Map>()
        .rooms()
        .iter()
        .skip(1)
        .map(Rect::center)
        .collect::<Vec<_>>();

    let mut rng = rltk::RandomNumberGenerator::new();
    for monster_pos in monster_positions {
        let roll = rng.roll_dice(1, 2);
        let glyph = match roll {
            1 => 'g',
            _ => 'o',
        };
        gs.ecs_mut().spawn((
            Monster,
            Position::from(monster_pos),
            Renderable::new(glyph, rltk::RED, None),
            Viewshed::new(8),
        ));
    }

    gs.ecs_mut().spawn((
        Player,
        Position::from(player_pos),
        Renderable::new('☺', rltk::YELLOW, None),
        Viewshed::new(8),
    ));

    rltk::main_loop(context, gs)
}
