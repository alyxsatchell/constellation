use std::{collections::HashMap, mem};

use crate::debug_logger::debug_log;
use crate::canvas::{Tile, Point};

pub trait Stencil{
    fn get_map(&self) -> &StencilMap;

    fn get_map_mut(&mut self) -> &mut StencilMap;

    //a default that can be overridden if desired
    fn generate_new_map(&self) -> HashMap<Point, Tile> {
        HashMap::new()
    }

    fn merge(&mut self, new_map: HashMap<Point, Tile>){
        self.get_map_mut().merge(new_map)
    }

    fn update(&mut self){
        self.merge(self.generate_new_map());
    }
}

#[derive(Debug)]
pub struct StencilMap{
    pub origin: Point,
    pub addition_map: HashMap<Point, Tile>,
    pub subtraction_map: Vec<Point>,
    pub current_map: HashMap<Point, Tile>
}

impl StencilMap{
    pub fn merge(&mut self, mut new_map: HashMap<Point, Tile>){
        let mut addition_map = HashMap::new();
        let mut subtraction_map = Vec::new();
        let current_map: HashMap<Point, Tile> = mem::replace(&mut self.current_map, HashMap::new());
        //checks what points of the old map are still relevant
        for i in current_map{
            let (point, tile) = i;
            //checks if the point is going to also be present in the next map
            //if it is not, then it needs to be in the sub map
            if !new_map.contains_key(&point){
                subtraction_map.push(point);
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
        // debug_log(&format!("{:?}", &new_map));
        for i in new_map{
            let (point, tile) = i;
            addition_map.insert(point, tile);
            self.current_map.insert(point, tile);
        }
        self.addition_map = addition_map;
        self.subtraction_map = subtraction_map;
    }

    pub fn translate(&mut self, translation: Point){
        debug_log("test");
        let new_map = self.translate_map(translation);
        self.merge(new_map);
        self.origin += translation;
    }

    fn translate_map(&self, translation: Point) -> HashMap<Point, Tile>{
        let mut new_map = HashMap::new();
        for i in &self.current_map{
            let (point, tile) = i;
            let new_point = *point + translation;
            new_map.insert(new_point, *tile);
        }
        debug_log(&format!("|{:?}||{:?}|", new_map, &self.current_map));
        return new_map
    }
}

// pub struct Transformation{
//     movement: Point,
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Translation{
//     pub origin: Point,
//     pub previous_origin: Option<Point>,
//     pub coord: Point,
//     pub tile: Tile
// }

// impl Translation{
//     pub fn new(coord: Point, origin: Point, previous_origin: Option<Point>, tile: Tile) -> Translation{
//         Translation { 
//             coord,
//             origin,
//             previous_origin,
//             tile
//         }
//     }

//     pub fn translation(&self) -> Point{
//         return self.coord + self.origin
//     }

//     pub fn previous_translation(&self) -> Point{
//         return self.coord + self.previous_origin.unwrap()
//     }
// }