pub mod canvas;
pub mod  stencil;
pub mod debug_logger;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{stencil::{Stencil, StencilMap}, canvas::{Tile, Color, Canvas, Point}, debug_logger::debug_log};


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
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let current_map = HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]);
        let add = HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]);
        let sub: HashMap<Point, Tile> = HashMap::new();
        let mut test_stencil_map: StencilMap = StencilMap{
            origin: Point { x: 0, y: 0 },
            addition_map: add,
            subtraction_map: sub,
            current_map,
        };
        let mut test_stencil = TestStencil{stencilmap: test_stencil_map};
        canvas.update(test_stencil.get_map_mut());
        canvas.draw();
    }

    #[test]
    fn test_animation(){
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let addition = Point{x:1, y: 0};
        let mut test_stencil_map = StencilMap::new(Point{x:0, y:0}, HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]));
        canvas.update(&mut test_stencil_map);
        canvas.draw();
        for _ in 0..100000000{
            let f: f32 = 1.1234251324124;
            let _ = f.sqrt();
        }
        for _ in 1..5{
            test_stencil_map.translate(addition);
            debug_log(&format!("{:?}", &test_stencil_map));
            canvas.update(&mut test_stencil_map);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
    }
    #[test]
    fn test_overlap(){
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let addition = Point{x:1, y: 0};
        let subtraction = Point{x:-1, y:0};
        let mut test_stencil_map = StencilMap::new(Point{x:0, y:0}, HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]));
        let mut test_stencil_map2 = StencilMap::new(Point{x:8, y:0}, HashMap::from([(Point{x:9,y:0}, tile), (Point{x:8, y:0}, tile), (Point{x:9, y:1}, tile), (Point{x:8,y:1}, tile)]));
        let stencil_vec = vec![&mut test_stencil_map, &mut test_stencil_map2];
        canvas.update_mult(stencil_vec);
        // canvas.update(&mut test_stencil_map);
        // canvas.update(&mut test_stencil_map2);
        canvas.draw();
        for _ in 0..100000000{
            let f: f32 = 1.1234251324124;
            let _ = f.sqrt();
        }
        for i in 1..9{
            test_stencil_map.translate(addition);
            test_stencil_map2.translate(subtraction);
            debug_log(&format!("{}{:?}", i, &test_stencil_map));
            let stencil_vec = vec![&mut test_stencil_map, &mut test_stencil_map2];
            canvas.update_mult(stencil_vec);
            // debug_log(&format!("{} {:?}", i, &test_stencil_map2));
            // canvas.update(&mut test_stencil_map2);
            // canvas.update(&mut test_stencil_map);
            // canvas.update(&test_stencil_map2);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
        for i in 10..19{
            test_stencil_map.translate(subtraction);
            test_stencil_map2.translate(addition);
            let stencil_vec = vec![&mut test_stencil_map, &mut test_stencil_map2];
            canvas.update_mult(stencil_vec);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
    }
}