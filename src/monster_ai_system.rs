use specs::prelude::*;
use crate::{Viewshed, components::Monster, Named};
use rltk::{Point, console};
pub struct MonsterAISystem{}

impl<'a> System<'a> for MonsterAISystem{
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Named>
    );

    fn run(&mut self, data: Self::SystemData){
        let (player_pos, viewshed, monster, name) = data;

        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join(){
            if viewshed.visible_tiles.contains(&*&player_pos){
                console::log(format!("{} does thing", name.name));
            }
        }
    }
}
