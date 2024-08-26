use specs::prelude::*;
use crate::{gamelog, player, CombatStats, Consumable, Potion, WantsToDrinkPotion, WantsToDropItem};

use super::{WantsToPickupItem,Name,InBackpack,Position,gamelog::GameLog};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem{
    #[allow(clippy::type_complexity)]
    type SystemData=(WriteStorage<'a,WantsToPickupItem>,
                     ReadExpect<'a,Entity>,
                     WriteStorage<'a,InBackpack>,
                     WriteStorage<'a,Position>,
                     WriteExpect<'a,GameLog>,
                     ReadStorage<'a,Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut want_pickup,player_entity,mut back_pack,mut position,mut gamelog,name) = data;
        for pickup in want_pickup.join(){
            position.remove(pickup.item);
            back_pack.insert(pickup.item, InBackpack{owner:pickup.collected_by}).expect("unexpect pickup");
            if *player_entity == pickup.collected_by{
                gamelog.entries.push(format!("You pickup the {}",name.get(pickup.item).unwrap().name));
            }
        }
        want_pickup.clear();
    }
}

pub struct ItemUseSystem{}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData=(Entities<'a>,
    WriteStorage<'a,CombatStats>,
    WriteStorage<'a,WantsToDrinkPotion>,
    ReadStorage<'a,Potion>,
    ReadExpect<'a,Entity>,
    WriteExpect<'a,GameLog>,
    ReadStorage<'a,Name>,
    ReadStorage<'a,Consumable>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities,mut stats,mut wants_drink,potions,player_entity,mut gamelog,name,mut consumables)= data;
        for (entity,stat,drink) in (&entities,&mut stats,&wants_drink).join(){
            let potion =potions.get(drink.potion);
            match potion {
                None =>{}
                Some(potion)=>{
                    let want_hp = i32::min(stat.max_hp,stat.hp+potion.heal_amount);
                    stat.hp = want_hp;
                    if entity == *player_entity{
                        gamelog.entries.push(format!("You drink the {}, healing {} hp.",name.get(drink.potion).unwrap().name,potion.heal_amount));
                    }
                    entities.delete(drink.potion).expect("Delete failed");
                }
            }
            //let comsumable = consumables.get()
        }
        wants_drink.clear();
    }
}

pub struct ItemDropSystem{}

impl<'a> System<'a> for ItemDropSystem  {
    type SystemData=(WriteStorage<'a,WantsToDropItem>,
                     WriteStorage<'a,Position>,
                     ReadExpect<'a,Entity>,
                     WriteExpect<'a,GameLog>,
                     ReadStorage<'a,Name>,
                     Entities<'a>,
                     WriteStorage<'a,InBackpack>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut want_drop,mut postions ,player_entity,mut gamelog,names,entities,mut backpack) = data;
        for (entity,want_drop) in (&entities,&want_drop).join(){
            if entity == *player_entity{
                let player_position = postions.get(*player_entity).unwrap();
                postions.insert(want_drop.item, Position{x:player_position.x,y:player_position.y}).expect("Unexpect insert position");
                gamelog.entries.push(format!("You drop the {}.",names.get(want_drop.item).unwrap().name));
                backpack.remove(want_drop.item).expect("Unexpect remove from backpack");
            }
        }
        want_drop.clear();
    }
}