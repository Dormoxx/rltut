use std::cmp::{max, min};

use crate::{components::*, maps::{TileType, xy_idx}};
use rltk::prelude::*;
use specs::*;


/*pub struct LeftMoverSystem{}

impl<'a> System<'a> for LeftMoverSystem{
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos) : Self::SystemData){
        for(_lefty,pos) in (&lefty, &mut pos).join(){
            pos.x -= 1;
            if pos.x < 0{
                pos.x = 79;
            }
        }
    }
}*/
