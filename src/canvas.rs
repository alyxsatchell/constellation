pub struct Canvas{
    strokes: StrokeList,
    size: (usize, usize),

}

pub struct StrokeList{
    stroke_list: Vec<Stroke>
}

pub struct Stroke{
    length: usize,
    color: Color,
}

pub struct Color{
    r: u8,
    g: u8,
    b: u8
}