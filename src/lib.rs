pub mod canvas;
pub mod  stencil;
pub mod debug_logger;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{stencil::{Stencil, StencilMap}, canvas::{Tile, Color, Canvas, Point}};


    struct TestStencil{
        stencilmap: StencilMap,
    }

    impl Stencil for TestStencil{
        fn get_map(&self) -> &StencilMap {
            &self.stencilmap
        }

        fn get_map_mut(&mut self) -> &mut StencilMap {
            &mut self.stencilmap
        }
    }
    #[test]
    fn test_stencil_draw(){
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true), 1);
        let current_map = HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]);
        let add = HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]);
        let sub: Vec<Point> = Vec::new();
        let test_stencil_map: StencilMap = StencilMap{
            origin: Point { x: 0, y: 0 },
            addition_map: add,
            subtraction_map: sub,
            current_map,
        };
        let test_stencil = TestStencil{stencilmap: test_stencil_map};
        canvas.update(test_stencil.get_map());
        canvas.draw();
    }

    #[test]
    fn test_animation(){
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true), 1);
        let addition = Point{x:1, y: 0};
        let current_map = HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]);
        let add = HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]);
        let sub: Vec<Point> = Vec::new();
        let mut test_stencil_map: StencilMap = StencilMap{
            origin: Point { x: 0, y: 0 },
            addition_map: add,
            subtraction_map: sub,
            current_map,
        };
        canvas.update(&test_stencil_map);
        canvas.draw();
        println!("{:?}", test_stencil_map);
        for _ in 0..100000000{
            let f: f32 = 1.1234251324124;
            let _ = f.sqrt();
        }
        for _ in 1..5{
            test_stencil_map.translate(addition);
            // let new_origin = origin + addition;
            // let new = HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]);
            // let test_stencil_map: StencilMap = vec![Translation::new(Point{x:0,y:0}, new_origin, Some(origin), tile), Translation::new(Point{x:0,y:1}, new_origin, Some(origin), tile), Translation::new(Point{x:1,y:0}, new_origin, Some(origin), tile), Translation::new(Point{x:1,y:1}, new_origin, Some(origin), tile)];
            canvas.update(&test_stencil_map);
            canvas.draw();
            println!("{:?}", &test_stencil_map);
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
    }
}