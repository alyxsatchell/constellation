use std::fmt::Display;

const BLACK_BG: Color = Color{
    background: true,
    r: 0,
    b: 0,
    g: 0
};
const BLACK_TILE: Tile = Tile{
    bg: Some(BLACK_BG),
    fg: None,
    character: ' '
};

pub struct Canvas{
    tiles: TileMap,
    size: (usize, usize),

}

impl Canvas{
    pub fn new(size: (usize, usize)) -> Canvas{
        Canvas { tiles: TileMap::new(size, BLACK_BG), size}
    }

    pub fn draw(&self){
        println!("{}", self.tiles.to_string());
        println!("{:?}", self.tiles);
    }
}

#[derive(Debug)]
pub struct TileMap{
    map: Vec<Vec<Tile>>
}

impl TileMap{
    pub fn new(size: (usize, usize), default_color: Color) -> TileMap{
        let mut map = Vec::new();
        for i in 0..size.1 {
            let mut tmp: Vec<Tile> = Vec::new();
            for j in 0..size.0{
                tmp.push(Tile::new(default_color));
            }
            map.push(tmp);
        }
        TileMap { map }
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
}

#[derive(Debug)]
pub struct Tile{
    bg: Option<Color>,
    fg: Option<Color>,
    character: char,
}

impl Tile{
    pub fn new(color: Color) -> Tile{
        Tile { bg: Some(color), fg: None, character: ' ' }
    }

    pub fn new_fg(bg: Color, fg: Color, character: char) -> Tile{
        Tile{
            bg: Some(bg),
            fg: Some(fg),
            character
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