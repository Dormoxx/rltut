//roguelike tutorial: https://bfnightly.bracketproductions.com/

mod components;
use components::*;
mod maps;
use maps::*;
mod player;
use player::*;
mod rect;
use rltk::{GameState, Rltk, RltkBuilder, RGB};
use specs::prelude::*;

pub struct State {
    pub ecs: World,
}
impl State {
    pub fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, rend) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph);
        }
    }
}

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gs)
}
