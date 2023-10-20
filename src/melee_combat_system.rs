use crate::{CombatStats, GameLog, Named, SufferDamage, WantsToMelee};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Named>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, GameLog>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_melee, names, combat_stats, mut inflict_damage, mut log) = data;

        for (_entity, wants_melee, name, stats) in
            (&entities, &wants_melee, &names, &combat_stats).join()
        {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();
                    let damage = i32::max(0, stats.power - target_stats.defense);
                    if damage == 0 {
                        log.entries.push(format!(
                            "{} is unable to damage {}",
                            &name.name, &target_name.name
                        ));
                    } else {
                        log.entries.push(format!(
                            "{} hits {} for {} hp!",
                            &name.name, &target_name.name, damage
                        ));
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    }
                }
            }
        }
        wants_melee.clear();
    }
}
