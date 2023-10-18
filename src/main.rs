//roguelike tutorial: https://bfnightly.bracketproductions.com/

mod components;
use components::*;
mod systems;
mod worldstate;
use rltk::{RltkBuilder, RGB};
use specs::prelude::*;
use worldstate::*;

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    //gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    /*for i in 0..10{
        gs.ecs
        .create_entity()
            .with(Position{x: i*7, y: 20})
            .with(Renderable{
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover{})
            .build();
    }*/

    rltk::main_loop(context, gs)
}
