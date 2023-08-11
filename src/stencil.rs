use crate::canvas::{Tile, Point};

pub type StencilMap = Vec<Transformation>;

pub trait Stencil{
    fn get_map(&self) -> StencilMap;
}

#[derive(Debug, Clone, Copy)]
pub struct Transformation{
    pub origin: Point,
    pub previous_origin: Option<Point>,
    pub coord: Point,
    pub tile: Tile
}

impl Transformation{
    pub fn new(coord: Point, origin: Point, previous_origin: Option<Point>, tile: Tile) -> Transformation{
        Transformation { 
            coord,
            origin,
            previous_origin,
            tile
        }
    }

    pub fn translation(&self) -> Point{
        return self.coord + self.origin
    }

    pub fn previous_translation(&self) -> Point{
        return self.coord + self.previous_origin.unwrap()
    }
}