use std::{fmt::Display, ops::{Add, AddAssign}, io::{stdout, Write}};

use termion::raw::IntoRawMode;

use crate::stencil::StencilMap;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point{
    pub x: i32,
    pub y: i32
}

impl From<Point> for (i32, i32) {
    fn from(point: Point) -> (i32, i32) {
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

impl AddAssign for Point{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

pub struct Canvas{
    tiles: TileMap,
    size: Point,
    origin: Point,
}

impl Canvas{
    pub fn new(origin: (usize, usize), size: (usize, usize), default_color: Color) -> Canvas{
        stdout().into_raw_mode().expect("Couldn't Initialize Canvas");
        let size: Point = Point { x: size.0 as i32, y: size.1 as i32 };
        let origin: Point = Point { x: origin.0 as i32, y: origin.1 as i32};
        stdout().write_fmt(format_args!("\x1b[2J")).expect("Failed To Clear");
        Canvas { 
            tiles: TileMap::new(size, default_color), 
            size,
            origin
        }
    }

    pub fn draw(&self){
        stdout().write_fmt(format_args!("\x1b[{};1H", self.origin.y + 1)).expect("Failed To Write");
        for line in self.tiles.get_lines(){
            stdout().write_fmt(format_args!("\x1b[{}C{}\n", self.origin.x, line)).expect("Failed To Write Line");
            stdout().write_fmt(format_args!("\r")).expect("Failed To Write");
        }
        stdout().flush().expect("Failed To Flush");
    }

    pub fn update(&mut self, stencilmap: &StencilMap){
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
        for _ in 0..size.y {
            let mut tmp: Vec<Tile> = Vec::new();
            for _ in 0..size.x{
                tmp.push(Tile::new(default_color, 0));
            }
            map.push(tmp);
        }
        TileMap { 
            map,
            default_tile: Tile::new(default_color, 0),
        }
    }

    pub fn get_lines(&self) -> Vec<String>{
        let mut string_vec = Vec::new();
        for row in &self.map{
            let mut tmp = String::new();
            for tile in row{
                tmp += &tile.to_string();
            }
            tmp += "\x1b[0m";
            string_vec.push(tmp);
        }
        return string_vec
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

    pub fn draw_stencilmap(&mut self, stencilmap: &StencilMap){
        //draw the addition
        for i in &stencilmap.addition_map{
            let (point, tile) = i;
            self.insert(*point, *tile);
        }
        for point in &stencilmap.subtraction_map{
            //might need a safeguard depending on reasonable assumptions
            self.insert(*point, self.default_tile);
        }
    }

    fn insert(&mut self, coord: Point, tile: Tile){
        let (x,y) = coord.into();
        if x < 0 || y < 0{
            return
        }
        if y as usize >= self.map.len(){
            return
        }
        if x as usize >= self.map[y as usize].len(){
            return
        }
        self.map[y as usize][x as usize] = tile;
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