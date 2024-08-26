use rltk::{GameState, Rltk, RGB, VirtualKeyCode,Point};
use specs::prelude::*;
use std::{cmp::{max, min}, process::id};
use specs_derive::Component;

mod map;
mod gui;
mod gamelog;

pub use map::*;
mod components;
pub use components::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
use visibility_system::VisibilitySystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod map_indexing_system;
use map_indexing_system::MapIdexingSystem;

mod melee_combat_system;
use melee_combat_system::MeleeCombatSystem;

mod damage_system;
use damage_system::DamageSystem;

mod inventory_system;
use inventory_system::ItemCollectionSystem;
use inventory_system::PotionUseSystem;
use inventory_system::ItemDropSystem;

mod spawner;
pub struct State{
    pub ecs: World
}

impl State {
    fn run_systems(&mut self){
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        
        
        let mut moa = MonsterAI{};
        moa.run_now(&self.ecs); 
        let mut map_index = MapIdexingSystem{};
        map_index.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem{};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem{};
        damage.run_now(&self.ecs);
        let mut pickup = ItemCollectionSystem{};
        pickup.run_now(&self.ecs);

        let mut potions = PotionUseSystem{};
        potions.run_now(&self.ecs);
        let mut drops = ItemDropSystem{};
        drops.run_now(&self.ecs);

        self.ecs.maintain();

    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        draw_map(&self.ecs, ctx);
        {
            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();
            let map = self.ecs.fetch::<Map>();
            let mut data = (&positions,&renderables).join().collect::<Vec<_>>();
            data.sort_by(|&a,&b|b.1.render_order.cmp(&a.1.render_order));
            for (pos,render) in data.iter(){
                let idx = map.xy_idx(pos.x, pos.y);
                if map.visible_tiles[idx]{ ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);}   
            }
            gui::draw_ui(&self.ecs, ctx);
        }
        let mut new_run_state;
        {
            let runstate = self.ecs.fetch::<RunState>();
            new_run_state = *runstate;
        }
        match new_run_state {
            RunState::PreRun =>{
                self.run_systems();
                self.ecs.maintain();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput =>{
                new_run_state = player_input(self, ctx);
            }
            RunState::PlayerTurn =>{
                self.run_systems();
                self.ecs.maintain();
                new_run_state =RunState::MonsterTurn;
            }
            RunState::MonsterTurn =>{
                self.run_systems();
                self.ecs.maintain();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::ShowInventory=>{
                let result = gui::show_inventory(self, ctx);
                match result.0{
                    gui::ItemMenuResult::Cancel=> new_run_state = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse =>{}
                    gui::ItemMenuResult::Selected =>{
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDrinkPotion>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToDrinkPotion{potion:item_entity}).expect("unable to insert intent");
                        new_run_state = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowDropItem=>{
                let result = gui::show_dropitem(self, ctx);
                match result.0{
                    gui::ItemMenuResult::Cancel=> new_run_state = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse =>{}
                    gui::ItemMenuResult::Selected =>{
                        let item_entity = result.1.unwrap();
                        let mut drop = self.ecs.write_storage::<WantsToDropItem>();
                        drop.insert(*self.ecs.fetch::<Entity>(), WantsToDropItem{item:item_entity}).expect("unable to insert drop ");
                        new_run_state = RunState::PlayerTurn;
                    }
                }
            }
        }
        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer =new_run_state;
        }
        DamageSystem::delete_the_dead(&mut self.ecs);
        
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
    .with_title("Roguelike Tutorial")
    .build()?;
    context.with_post_scanlines(true);
    let mut gs = State{
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlockTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToDrinkPotion>();
    gs.ecs.register::<WantsToDropItem>();
    gs.ecs.register::<Consumable>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x,player_y) = map.rooms[0].center();
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);
    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    for room in map.rooms.iter().skip(1){
        spawner::spawn_room(&mut gs.ecs, room);
    }
    gs.ecs.insert(Point::new(player_x, player_y));

    gs.ecs.insert(map);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog{entries: vec!["Welcome to Rusty Roguelike".to_string()]});
    
    rltk::main_loop(context, gs)
}
