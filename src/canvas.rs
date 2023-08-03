use std::fmt::Display;

pub struct Canvas{
    strokes: StrokeList,
    size: (usize, usize),

}

pub struct StrokeList{
    stroke_list: Vec<Stroke>
}

impl StrokeList{
    pub fn new() -> StrokeList{
        StrokeList { stroke_list: Vec::new() }
    }

    //temp functionality to test strokes
    pub fn insert(&mut self, stroke: Stroke){
        self.stroke_list.push(stroke);
    }

    pub fn draw(&self){
        for stroke in &self.stroke_list{
            print!("{}", stroke.draw());
        }
    }
}

pub struct Stroke{
    length: usize,
    color: Color,
}

impl Stroke{
    pub fn new(color: Color, length: usize) -> Stroke{
        Stroke { 
            length, 
            color
        }
    }

    pub fn draw(&self) -> String{
        let space = " ".repeat(self.length);
        return format!("{}{}\x1b[0m", self.color, space)
    }
}

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