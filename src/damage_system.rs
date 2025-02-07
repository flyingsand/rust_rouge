use rltk::console;
use specs::prelude::*;
use crate::{gamelog::GameLog, player};

use super::{CombatStats, SufferDamage,Player,Name};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem{
    type SystemData =( WriteStorage<'a,CombatStats>,
                       WriteStorage<'a,SufferDamage>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats,mut damage) = data;
        for (mut stats,damage) in (&mut stats,&damage).join(){
            stats.hp -= damage.amount.iter().sum::<i32>();
        }
        damage.clear();
    }
}

impl DamageSystem {
    pub fn delete_the_dead(ecs: &mut World){
        let mut dead : Vec<Entity> = Vec::new();
        {   
            let combat_stats = ecs.read_storage::<CombatStats>();
            let entities = ecs.entities();
            let player = ecs.read_storage::<Player>();
            let mut log = ecs.write_resource::<GameLog>();
            let names = ecs.read_storage::<Name>();
            for (entity,stats) in (&entities,&combat_stats).join(){
                if stats.hp <1 {
                    let _p =player.get(entity);
                    match _p {
                        None => {
                            let victim_name = names.get(entity);
                            if let Some(victim_name) = victim_name{
                                log.entries.push(format!("{} is dead", &victim_name.name));
                            }
                            dead.push(entity);
                        },
                        Some(_) => console::log("ypu are dead")
                    }   
                }
            }
        }
        for victim in dead{
            ecs.delete_entity(victim).expect("Unable to delete");
        }
    }
}