use crate::{map_builder::drunkard, prelude::*};
use super::MapArchitect;

const STAGGER_DISTANCE: usize= 400;
const NUM_TILES: usize=(SCREEN_HEIGHT * SCREEN_WIDTH) as usize;
const DESIRED_FLOOR:usize = NUM_TILES/3;

pub struct DrunkardsWalkArchitect {}

impl  DrunkardsWalkArchitect {
    
    fn drunkard(
    &mut self,
    start:&Point,
    rng :&mut RandomNumberGenerator,
    map:&mut Map
    ){
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;
        loop{
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;
            match  rng.range(0, 4) {
                0 => drunkard_pos.x -=1,
                1 => drunkard_pos.x +=1,
                2 => drunkard_pos.y -=1,
                _=> drunkard_pos.y +=1,
            }
            if !map.in_bounds(drunkard_pos) {
                break;
            }
            distance_staggered +=1;
            if distance_staggered > STAGGER_DISTANCE{
                break;
            }
        }

    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb=MapBuilder{
        map : Map::new(),
        rooms : Vec::new(),
        monster_spawns : Vec::new(),
        player_start : Point::zero(),
        amulet_start : Point::zero(),
        theme: super::themes::DungeonTheme::new()
        };

        mb.fill(TileType::Wall);
        let centor = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        self.drunkard(&centor, rng, &mut mb.map);
        while mb.map.tiles.iter()
        .filter(|t| **t== TileType::Floor).count() <DESIRED_FLOOR{
            self.drunkard(&Point::new(
                rng.range(0, SCREEN_WIDTH), 
                rng.range(0, SCREEN_HEIGHT)), rng, &mut mb.map);
        }

        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH, 
            SCREEN_HEIGHT, &vec![mb.map.point2d_to_index(centor)],
             &mb.map,
            1024.0);
        dijkstra_map.map.iter().enumerate()
        .filter(|(_, distance)| * distance > &2000.0)
        .for_each(|(idx, _)|mb.map.tiles[idx] = TileType::Wall);

        mb.monster_spawns = mb.spawn_monsters(&centor, rng);
        mb.player_start = centor;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}