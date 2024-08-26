use std::{fmt::format, process::id};

use specs::prelude::*;

use crate::{RunState, WantsToMelee};

use super::{Viewshed, Position, Map, Monster,Name};
use rltk::{field_of_view, Point, console};

pub struct MonsterAI{}

impl<'a> System<'a> for MonsterAI{
    
    
    type SystemData=(ReadStorage<'a,Monster>,
                    WriteStorage<'a,Viewshed>,
                    ReadExpect<'a,Point>,
                    WriteExpect<'a,Map>,
                    WriteStorage<'a,Position>,
                    Entities<'a>,
                    ReadExpect<'a,Entity>,
                    WriteStorage<'a,WantsToMelee>,
                    ReadExpect<'a,RunState>
                );
    
    fn run(&mut self, data: Self::SystemData) {
        let (monster,mut view,player_pos,mut map,mut pos,entities,player_entity,mut wants_to_melee,run_state) = data;
        if *run_state != RunState::MonsterTurn {return;}
        let mut amount_of_monster = 0;
        for (entity,_monster,mut view,mut pos) in (&entities,&monster,&mut view,&mut pos).join(){
            amount_of_monster+=1;
            if view.visible_tiles.contains(&*player_pos){
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);

                if distance<1.5 {
                    wants_to_melee.insert(entity, WantsToMelee{target:*player_entity})
                    .expect("Unable to insert attack");
                    continue;
                }
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y),
                    map.xy_idx(player_pos.x, player_pos.y),
                    &mut *map
                );
                if path.success && path.steps.len() >1 {
                    let mut idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = false;
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = true;
                    view.dirty = true;
                }
            }
        }
        console::log(format!("still {} monster in thetuto",amount_of_monster));
    }
}