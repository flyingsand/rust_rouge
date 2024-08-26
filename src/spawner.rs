use rltk::{ RGB, RandomNumberGenerator };
use specs::prelude::*;
use crate::{MAPCOUNT, MAPHEIGHT, MAPWIDTH};

use super::{CombatStats, Player, Renderable, Name, Position, Viewshed, Monster, BlockTile,Rect,Potion,Item};

const MAX_MONSTERS: i32 = 4;
const MAX_ITEM: i32 = 2;

pub fn player(ecs:&mut World,player_x:i32,player_y:i32)->Entity{
        ecs
        .create_entity()
        .with(Position{ x:player_x,y:player_y})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order:0
        })
        .with(Player{})
        .with(Viewshed{visible_tiles:Vec::new(),range:8,dirty:true})
        .with(Name{name:"Player".to_string()})
        .with(CombatStats{max_hp:30,hp:30,power:10,defense:3})
        .build()
    
}

pub fn spawn_room(ecs:&mut World, room:&Rect){
    let mut monster_spawn_point: Vec<usize> = Vec::new();
    let mut item_spawn_point: Vec<usize> = Vec::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let monsters_number = rng.roll_dice(1, MAX_MONSTERS+2)-3;
        let item_number = rng.roll_dice(1, MAX_ITEM+2)-3;
        for _i in 0..monsters_number{
            let mut added = false;
            while !added {
                let x = room.x1 + rng.roll_dice(1, room.x2-room.x1);
                let y = room.y1 + rng.roll_dice(1, room.y2-room.y1);
                let idx = y as usize *MAPWIDTH+x as usize;
                if !monster_spawn_point.contains(&idx){
                    monster_spawn_point.push(idx);
                    added = true;
                }
            }
        }

        for _i in 0..item_number{
            let mut added = false;
            while !added {
                let x = room.x1 + rng.roll_dice(1, room.x2-room.x1);
                let y = room.y1 + rng.roll_dice(1, room.y2-room.y1);
                let idx = y as usize *MAPWIDTH+x as usize;
                if !item_spawn_point.contains(&idx){
                    item_spawn_point.push(idx);
                    added = true;
                }
            }
        }
    }
    for idx in monster_spawn_point.iter(){
        let x = idx%MAPWIDTH;
        let y = idx/MAPWIDTH;
        random_monster(ecs, x as i32, y as i32);
    }

    for idx in item_spawn_point.iter(){
        let x = idx%MAPWIDTH;
        let y = idx/MAPWIDTH;
        health_potion(ecs, x as i32, y as i32);
    }
}
pub fn random_monster(ecs:&mut World,x:i32,y:i32)->Entity{
    let ran: i32;
    {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    ran = rng.roll_dice(1, 2);
    };
    match ran {
        1 => orc(ecs, x, y),
        _ => goblin(ecs, x, y)
    }
}

fn orc(ecs:&mut World,x:i32,y:i32)->Entity{monster(ecs, x, y, rltk::to_cp437('o'), "Orc")}
fn goblin(ecs:&mut World,x:i32,y:i32)->Entity{monster(ecs, x, y, rltk::to_cp437('g'), "Goblin")}


fn monster<T:ToString>(ecs:&mut World,x:i32,y:i32,glyph:rltk::FontCharType, name:T)->Entity{
            ecs
            .create_entity()
            .with(Position{ x,y})
            .with(Renderable{
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
                render_order:1
            })
            .with(Viewshed{visible_tiles:Vec::new(),range:8,dirty:true})
            .with(Monster{})
            .with(Name{name:name.to_string()})
            .with(BlockTile{})
            .with(CombatStats{max_hp:16,hp:16,power:4,defense:1})
            .build()
}

fn health_potion(ecs:&mut World,x:i32,y:i32)->Entity{
    ecs
    .create_entity()
    .with(Position{x,y})
    .with(Renderable{
        glyph:rltk::to_cp437('¡'),
        fg: RGB::named(rltk::MAGENTA),
        bg: RGB::named(rltk::BLACK),
        render_order:2
    })
    .with(Name{name:"Health Potion".to_string()})
    .with(Item{})
    .with(Potion{heal_amount:8})
    .build()
}