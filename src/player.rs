use rltk::{console, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use crate::{gamelog::{self, GameLog}, CombatStats, Item, Viewshed, WantsToMelee, WantsToPickupItem};

use super::{Position, Player, TileType, Map, State,RunState};
use std::{cmp::{max, min}, fmt::format};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    
    let map = ecs.fetch::<Map>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities(); 
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity,_player, pos,view) in (&entities,&mut players, &mut positions,&mut viewsheds).join() {
        if pos.x + delta_x < 1 || pos.x + delta_x > map.width-1 || pos.y + delta_y < 1 || pos.y + delta_y > map.height-1 { return; }
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        
        for potential_target in map.tile_content[destination_idx].iter(){
            let target = combat_stats.get(*potential_target);
            match target {
                Some(t)=>{
                    console::log(format!("我攻击了！"));
                    wants_to_melee
                    .insert(entity,WantsToMelee{target:*potential_target})
                    .expect("Add target failed");
                    return;
                }
                None =>{}
            }
        }
        
        if !map.blocked[destination_idx] {
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            view.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }

}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => {return RunState::AwaitingInput} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            
            VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::C => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::F => get_item(&mut gs.ecs),
            VirtualKeyCode::B =>{return RunState::ShowInventory;},
            VirtualKeyCode::G =>{return RunState::ShowDropItem;}
            _ => {return RunState::AwaitingInput}
        },
    }
    RunState::PlayerTurn
}
fn get_item(ecs: &mut World){
    let entities = ecs.entities();
    let player_pos = ecs.fetch::<Point>();
    let mut player_entity=ecs.fetch::<Entity>();
    let items=ecs.read_storage::<Item>();
    let positions=ecs.read_storage::<Position>();

    let mut gamelog = ecs.fetch_mut::<GameLog>();
    let mut be_pick: Option<Entity> = None;
    for (entity,_item,position) in (&entities,&items,&positions).join(){
        if position.x==player_pos.x && position.y == player_pos.y{
            be_pick = Some(entity);
        }
    }
    match be_pick {
        None =>{gamelog.entries.push("There is nothing here to pick up".to_string())},
        Some(item) =>{
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup.insert(*player_entity, WantsToPickupItem{collected_by:*player_entity,item}).expect("Unable to insert want to pickup");
        }
    }
}