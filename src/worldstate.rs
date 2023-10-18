use crate::systems::*;
use crate::components::*;
use rltk::prelude::*;
use specs::prelude::*;
pub struct State {
    pub ecs: World,
}
impl State{
    pub fn run_systems(&mut self){
        let mut lms = LeftMoverSystem{};
        lms.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for(pos, rend) in (&positions, &renderables).join(){
            ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph);
        }
    }
}