use rltk::console;
use specs::prelude::*;
use super::{Map, Position, BlockTile};

pub struct MapIdexingSystem{

}


impl<'a> System<'a> for MapIdexingSystem{
    type SystemData=(
        WriteExpect<'a,Map>,
        ReadStorage<'a,Position>,
        ReadStorage<'a,BlockTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let(mut map,pos,block,entity)=data;
        map.populate_blocked();
        map.clear_content_index();
        for(pos ,entity) in (&pos,&entity).join(){
            let idx = map.xy_idx(pos.x, pos.y);
            let _p:Option<&BlockTile> = block.get(entity);
            if let Some(_p) = _p{
                map.blocked[idx]=true;
            }
            map.tile_content[idx].push(entity);
        }
    }
}