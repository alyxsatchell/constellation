use std::{fmt::Display, ops::Add};

use crate::stencil::{StencilMap, Transformation};

const BLACK_BG: Color = Color{
    background: true,
    r: 0,
    b: 0,
    g: 0
};
const BLACK_TILE: Tile = Tile{
    id: 0,
    bg: Some(BLACK_BG),
    fg: None,
    character: ' '
};

#[derive(Debug, Copy, Clone)]
pub struct Point{
    pub x: usize,
    pub y: usize
}

impl From<Point> for (usize, usize) {
    fn from(point: Point) -> (usize, usize) {
        let Point { x, y } = point;
        (x, y)
    }
}

impl Add for Point{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

pub struct Canvas{
    tiles: TileMap,
    size: Point,

}

impl Canvas{
    pub fn new(size: (usize, usize), default_color: Color) -> Canvas{
        let size: Point = Point { x: size.0, y: size.1 };
        Canvas { 
            tiles: TileMap::new(size, default_color), 
            size}
    }

    pub fn draw(&self){
        println!("{}", self.tiles.to_string());
    }

    pub fn update(&mut self, stencilmap: StencilMap){
        self.tiles.draw_stencilmap(stencilmap);
    }
}

#[derive(Debug)]
pub struct TileMap{
    default_tile: Tile,
    map: Vec<Vec<Tile>>
}

impl TileMap{
    pub fn new(size: Point, default_color: Color) -> TileMap{
        let mut map = Vec::new();
        for i in 0..size.y {
            let mut tmp: Vec<Tile> = Vec::new();
            for j in 0..size.x{
                tmp.push(Tile::new(default_color, 0));
            }
            map.push(tmp);
        }
        TileMap { 
            map,
            default_tile: Tile::new(default_color, 0),
        }
    }

    pub fn to_string(&self) -> String{
        let mut string = String::new();
        for row in &self.map{
            for tile in row{
                string += &tile.to_string();
            }
            string += "\x1b[0m";
            string.push('\n');
        }
        string.pop();
        string
    }

    pub fn draw_stencilmap(&mut self, stencilmap: StencilMap){
        for trans in stencilmap{
            match trans.previous_origin{
                Some(_) => self.replace(trans),
                None => self.insert(trans.translation(), trans.tile),
            }
        }
    }

    fn replace(&mut self, trans: Transformation){
        let (x, y) = trans.origin.into();
        let id = trans.tile.id;
        self.insert(trans.translation(), trans.tile);
        let default_tile = &self.default_tile;
        if id == self.map[y][x].id{
            self.insert(trans.previous_translation(), *default_tile);
        }
    }

    fn insert(&mut self, coord: Point, tile: Tile){
        let (x,y) = coord.into();
        if y >= self.map.len(){
            return
        }
        if x >= self.map[y].len(){
            return
        }
        self.map[y][x] = tile;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Tile{
    id: i32,
    bg: Option<Color>,
    fg: Option<Color>,
    character: char,
}

impl Tile{
    pub fn new(color: Color, id: i32) -> Tile{
        Tile { bg: Some(color), fg: None, character: ' ', id }
    }

    pub fn new_fg(bg: Color, fg: Color, character: char, id: i32) -> Tile{
        Tile{
            bg: Some(bg),
            fg: Some(fg),
            character,
            id
        }
    }

    pub fn to_string(&self) -> String{
        let mut val = String::new();
        match self.bg{
            Some(bg_color) => {
                val += &bg_color.write()
            },
            None => (),
        }
        match self.fg{
            Some(fg_color) => {
                val += &fg_color.write()
            },
            None => (),
        }
        val.push(self.character);
        return val
    }
}

impl PartialEq for Tile{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.bg == other.bg && self.fg == other.fg
    }
}

#[derive(Clone,Copy, Debug)]
pub struct Color{
    background: bool,
    r: u8,
    g: u8,
    b: u8
}

impl Color{
    pub fn new(r: u8, g: u8, b: u8, background: bool) -> Color{
        Color{
            r,
            g,
            b,
            background
        }
    }

    pub fn write(&self) -> String{
        let background = match self.background{
            true => "4",
            false => "3",
        };
        return format!("\x1b[{}8;2;{};{};{}m", background, self.r, self.g, self.b)
    }
}

impl Display for Color{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let background = match self.background{
            true => "4",
            false => "3",
        };
        write!(f, "\x1b[{}8;2;{};{};{}m", background, self.r, self.g, self.b)
    }
}

impl PartialEq for Color{
    fn eq(&self, other: &Self) -> bool {
        self.background == other.background && self.r == other.r && self.g == other.g && self.b == other.b
    }
}