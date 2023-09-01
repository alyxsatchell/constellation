use std::hash::Hash;
use std::{collections::HashMap, mem};

use crate::debug_logger::debug_log;
use crate::canvas::{Tile, Position};

pub trait Stencil{
    fn get_map(&self) -> &StencilMap;

    fn get_map_mut(&mut self) -> &mut StencilMap;

    //a default that can be overridden if desired
    fn generate_new_map(&self) -> HashMap<Position, Tile> {
        HashMap::new()
    }

    fn merge(&mut self, new_map: HashMap<Position, Tile>){
        self.get_map_mut().merge(new_map)
    }

    fn update(&mut self){
        self.merge(self.generate_new_map());
    }
}

#[derive(Debug)]
pub struct StencilMap{
    pub origin: Position,
    pub addition_map: HashMap<Position, Tile>,
    pub subtraction_map: HashMap<Position, Tile>,
    pub current_map: HashMap<Position, Tile>
}

impl StencilMap{
    pub fn new(origin: Position, map: HashMap<Position, Tile>) -> StencilMap{
        let mut sm = StencilMap { 
            origin, 
            addition_map: HashMap::new(), 
            subtraction_map: HashMap::new(), 
            current_map: HashMap::new()
        };
        sm.merge(map);
        sm
    }

    pub fn merge(&mut self, mut new_map: HashMap<Position, Tile>){
        let mut addition_map = HashMap::new();
        let mut subtraction_map = HashMap::new();
        let current_map: HashMap<Position, Tile> = mem::replace(&mut self.current_map, HashMap::new());
        //checks what points of the old map are still relevant
        for i in current_map{
            let (point, tile) = i;
            //checks if the point is going to also be present in the next map
            //if it is not, then it needs to be in the sub map
            if !new_map.contains_key(&point){
                // subtraction_map.push(point);
                subtraction_map.insert(point, tile);
            }
            //if it is present, it then checks if the tile has changed to be added to the add map
            else{
                let new_tile = new_map.remove(&point).unwrap();
                if tile != new_tile{
                    addition_map.insert(point, new_tile);
                    self.current_map.insert(point, new_tile);
                }
                else{
                    self.current_map.insert(point, tile);
                }
            }
        }
        //checks the remaining points that are new to new_map
        for i in new_map{
            let (point, tile) = i;
            addition_map.insert(point, tile);
            self.current_map.insert(point, tile);
        }
        self.addition_map = addition_map;
        self.subtraction_map = subtraction_map;
    }

    pub fn translate(&mut self, translation: Position){
        let new_map = self.translate_map(translation);
        self.merge(new_map);
        self.origin += translation;
    }

    fn translate_map(&self, translation: Position) -> HashMap<Position, Tile>{
        let mut new_map = HashMap::new();
        for i in &self.current_map{
            let (point, tile) = i;
            let new_point = *point + translation;
            new_map.insert(new_point, *tile);
        }
        return new_map
    }
}