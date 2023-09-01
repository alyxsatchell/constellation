use std::ops::{Add, AddAssign};

pub fn mod_add(num1: i32, num2: i32, max: i32) -> i32{
    return (num1 + num2) % max
}

#[derive(Debug, Copy, Clone)]
pub struct Point{
    x: f64,
    y: f64
}

impl From<Point> for (f64, f64) {
    fn from(point: Point) -> (f64, f64) {
        let Point { x, y } = point;
        (x, y)
    }
}

impl From<(f64, f64)> for Point{
    fn from(value: (f64, f64)) -> Self {
        Point{x: value.0, y: value.1}
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

pub struct Polygon<'a>{
    vertex_count: usize,
    vertices: VertexList<'a>,
}

impl<'a> Polygon<'a>{
    pub fn new(point_list: Vec<Point>) -> Polygon<'a>{
        let vertex_count = point_list.len();
        let first_point = point_list[0];
        let middle_point = point_list[1];
        let end_point = point_list[2];
        let side1 = Side::new(first_point.into(), middle_point.into());
        let side2 = Side::new(middle_point.into(), end_point.into());
        let mut seed_vertex = Vertex::new(side1, side2);
        let mut current_vertex = &mut seed_vertex;
        for i in 1..vertex_count{
            //let side1 = Side::new(point_list[mod_add(i as i32, -1, (vertex_count - 1) as i32) as usize].into(), point_list[i].into());
            let side2 = Side::new(point_list[i].into(), point_list[mod_add(i as i32, 1, (vertex_count - 1) as i32) as usize].into());//make new sides, turn into new vertexes, make new previouses so on so forth
            let mut next_vertex = current_vertex.extend(side2).unwrap();
            next_vertex.previous_vertex = Some(&current_vertex);
            current_vertex = next_vertex;
        }
        Polygon { vertex_count, vertices: VertexList{first_vertex: seed_vertex} }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Side{
    pub start_point: Point,
    pub end_point: Point,
    pub angle: f64,
    pub slope: f64,
    pub length: f64
}

impl Side{
    pub fn new(start_point: (f64, f64), end_point: (f64, f64)) -> Side{
        let start_point: Point = start_point.into();
        let end_point: Point = end_point.into();
        let slope = (start_point.y - end_point.y) / (start_point.x - end_point.x);
        let angle = slope.atan();
        let length = ((start_point.y - end_point.y) / angle.sin()).abs();
        Side { start_point, end_point, angle, slope, length}
    }
}

pub struct Vertex<'a>{
    previous_vertex: Option<&'a Vertex<'a>>,
    next_vertex: Option<Box<Vertex<'a>>>,
    previous_side: Side,
    next_side: Side
}

impl<'a> Vertex<'a>{
    pub fn new(previous_side: Side, next_side: Side) -> Vertex<'a>{
        Vertex { previous_vertex: None, next_vertex: None, previous_side, next_side}
    }

    pub fn extend(&'a mut self, next_side: Side) -> Option<&mut Vertex<'a>>{
        let vertex : Vertex<'a> = Vertex { previous_vertex: None, next_vertex: None, previous_side: self.next_side, next_side};
        self.next_vertex = Some(Box::new(vertex));
        let val: Option<&mut Vertex<'a>> =  match &mut self.next_vertex{
            Some(v) => {
                Some(&mut *v)
            },
            None => None,
        };
        return val
    }
}

pub struct VertexList<'a>{
    first_vertex: Vertex<'a>,
}

pub struct Ellipse{

}

pub enum Shape<'a>{
    Polygon(Polygon<'a>),
    Ellipse(Ellipse),
    Circle(Ellipse)
}