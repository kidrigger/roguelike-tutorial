use rltk::Rect;
use roguelike::{
    component::{CombatStats, Monster, Name, Player, Position, Renderable, TileBlock, Viewshed},
    resource::{Map, PlayerPosition, RunState},
    state::State,
};

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // context.with_post_scanlines(true);

    let mut gs = State::new();
    let player_pos = gs.res().fetch::<Map>().rooms().first().unwrap().center();
    gs.res_mut().insert(PlayerPosition(player_pos));
    gs.res_mut().insert(RunState::Running);

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
        let (glyph, name) = match roll {
            1 => ('g', "goblin"),
            _ => ('o', "orc"),
        };
        gs.ecs_mut().spawn((
            Monster,
            Name::from(name),
            Position::from(monster_pos),
            Renderable::new(glyph, rltk::RED, None),
            Viewshed::new(8),
            CombatStats::new(30, 2, 5),
            TileBlock,
        ));
    }

    gs.ecs_mut().spawn((
        Player,
        Name::from("Player"),
        Position::from(player_pos),
        Renderable::new('☺', rltk::YELLOW, None),
        CombatStats::new(16, 1, 4),
        Viewshed::new(8),
    ));

    rltk::main_loop(context, gs)
}
