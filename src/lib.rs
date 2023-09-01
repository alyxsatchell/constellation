pub mod canvas;
pub mod  stencil;
pub mod debug_logger;
pub mod math;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::{Mutex, Arc}};

    use crate::{stencil::{Stencil, StencilMap}, canvas::{Tile, Color, Canvas, Position}, debug_logger::debug_log, math::Side};

    struct TestTraitStruct{
        map: StencilMap,
    }

    impl TestTrait for TestTraitStruct{
        fn get_map_mut2(&mut self) -> &mut StencilMap {
            &mut self.map
        }
    }

    trait TestTrait{
        fn get_map_mut2(&mut self) -> &mut StencilMap;
    }

    impl Stencil for dyn TestTrait{
        fn get_map(&self) -> &StencilMap {
            todo!()
        }

        fn get_map_mut(&mut self) -> &mut StencilMap {
            self.get_map_mut2()
        }
    }

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
        let current_map = HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]);
        let add = HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]);
        let sub: HashMap<Position, Tile> = HashMap::new();
        let mut test_stencil_map: StencilMap = StencilMap{
            origin: Position { x: 0, y: 0 },
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
        let addition = Position{x:1, y: 0};
        let mut test_stencil_map = StencilMap::new(Position{x:0, y:0}, HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]));
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
    fn test_overlap_single(){
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let addition = Position{x:1, y: 0};
        let subtraction = Position{x:-1, y:0};
        let mut test_stencil_map = StencilMap::new(Position{x:0, y:0}, HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]));
        let mut test_stencil_map2 = StencilMap::new(Position{x:8, y:0}, HashMap::from([(Position{x:9,y:0}, tile), (Position{x:8, y:0}, tile), (Position{x:9, y:1}, tile), (Position{x:8,y:1}, tile)]));
        // let stencil_vec = vec![&mut test_stencil_map, &mut test_stencil_map2];
        // canvas.update_mult(stencil_vec);
        canvas.update(&mut test_stencil_map);
        canvas.update(&mut test_stencil_map2);
        canvas.draw();
        for _ in 0..100000000{
            let f: f32 = 1.1234251324124;
            let _ = f.sqrt();
        }
        for i in 1..9{
            test_stencil_map.translate(addition);
            test_stencil_map2.translate(subtraction);
            debug_log(&format!("{}{:?}", i, &test_stencil_map));
            // let stencil_vec = vec![&mut test_stencil_map, &mut test_stencil_map2];
            // canvas.update_mult(stencil_vec);
            // debug_log(&format!("{} {:?}", i, &test_stencil_map2));
            // canvas.update(&mut test_stencil_map2);
            canvas.update(&mut test_stencil_map);
            canvas.update(&mut test_stencil_map2);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
        for i in 10..19{
            test_stencil_map.translate(subtraction);
            test_stencil_map2.translate(addition);
            canvas.update(&mut test_stencil_map);
            canvas.update(&mut test_stencil_map2);
            // let stencil_vec = vec![&mut test_stencil_map, &mut test_stencil_map2];
            // canvas.update_mult(stencil_vec);
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
        let addition = Position{x:1, y: 0};
        let subtraction = Position{x:-1, y:0};
        let mut test_stencil_map = StencilMap::new(Position{x:0, y:0}, HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]));
        let mut test_stencil_map2 = StencilMap::new(Position{x:8, y:0}, HashMap::from([(Position{x:9,y:0}, tile), (Position{x:8, y:0}, tile), (Position{x:9, y:1}, tile), (Position{x:8,y:1}, tile)]));
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

    #[test]
    fn test_overlap_stencil(){
        let addition = Position{x:1, y: 0};
        let subtraction = Position{x:-1, y:0};
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let mut TestStencil1 = TestStencil{stencilmap: StencilMap::new(Position{x:0, y:0}, HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]))};
        let mut TestStencil2 = TestStencil{stencilmap: StencilMap::new(Position{x:8, y:0}, HashMap::from([(Position{x:9,y:0}, tile), (Position{x:8, y:0}, tile), (Position{x:9, y:1}, tile), (Position{x:8,y:1}, tile)]))};
        {
            let stencil_vec: Vec<&mut dyn Stencil> = vec![&mut TestStencil1, &mut TestStencil2];
            canvas.update_stencils(stencil_vec);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
        for i in 1..9{
            TestStencil1.stencilmap.translate(addition);
            TestStencil2.stencilmap.translate(subtraction);
            let stencil_vec: Vec<&mut dyn Stencil> = vec![&mut TestStencil1, &mut TestStencil2];
            canvas.update_stencils(stencil_vec);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
        for i in 10..19{
            TestStencil1.stencilmap.translate(subtraction);
            TestStencil2.stencilmap.translate(addition);
            let stencil_vec: Vec<&mut dyn Stencil> = vec![&mut TestStencil1, &mut TestStencil2];
            canvas.update_stencils(stencil_vec);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
    }

    #[test]
    fn test_overlap_mutex(){
        let addition = Position{x:1, y: 0};
        let subtraction = Position{x:-1, y:0};
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let mut TestStencil1: Mutex<Box<dyn Stencil>> = Mutex::new(Box::new(TestStencil{stencilmap: StencilMap::new(Position{x:0, y:0}, HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]))}));
        let mut TestStencil2: Mutex<Box<dyn Stencil>> = Mutex::new(Box::new(TestStencil{stencilmap: StencilMap::new(Position{x:8, y:0}, HashMap::from([(Position{x:9,y:0}, tile), (Position{x:8, y:0}, tile), (Position{x:9, y:1}, tile), (Position{x:8,y:1}, tile)]))}));
        //set up test to try out mutex support
        let stencil_vec: Vec<Mutex<Box<dyn Stencil>>> = vec![TestStencil1, TestStencil2];
        canvas.update_stencils_mutex(&stencil_vec);
        canvas.draw();
        for _ in 0..100000000{
            let f: f32 = 1.1234251324124;
            let _ = f.sqrt();
        }
        for i in 1..9{
            stencil_vec[0].lock().unwrap().get_map_mut().translate(addition);
            stencil_vec[1].lock().unwrap().get_map_mut().translate(subtraction);
            canvas.update_stencils_mutex(&stencil_vec);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
        for i in 10..19{
            stencil_vec[0].lock().unwrap().get_map_mut().translate(subtraction);
            stencil_vec[1].lock().unwrap().get_map_mut().translate(addition);
            canvas.update_stencils_mutex(&stencil_vec);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
    }

    #[test]
    fn test_overlap_arc_mutex(){
        let addition = Position{x:1, y: 0};
        let subtraction = Position{x:-1, y:0};
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let mut TestStencil1: Arc<Mutex<Box<dyn Stencil>>> = Arc::new(Mutex::new(Box::new(TestStencil{stencilmap: StencilMap::new(Position{x:0, y:0}, HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]))})));
        let mut TestStencil2: Arc<Mutex<Box<dyn Stencil>>> = Arc::new(Mutex::new(Box::new(TestStencil{stencilmap: StencilMap::new(Position{x:8, y:0}, HashMap::from([(Position{x:9,y:0}, tile), (Position{x:8, y:0}, tile), (Position{x:9, y:1}, tile), (Position{x:8,y:1}, tile)]))})));
        //set up test to try out mutex support
        let stencil_vec: Vec<Arc<Mutex<Box<dyn Stencil>>>> = vec![TestStencil1, TestStencil2];
        canvas.update_stencils_arc_mutex(&stencil_vec);
        canvas.draw();
        for _ in 0..100000000{
            let f: f32 = 1.1234251324124;
            let _ = f.sqrt();
        }
        for i in 1..9{
            stencil_vec[0].lock().unwrap().get_map_mut().translate(addition);
            stencil_vec[1].lock().unwrap().get_map_mut().translate(subtraction);
            canvas.update_stencils_arc_mutex(&stencil_vec);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
        for i in 10..19{
            stencil_vec[0].lock().unwrap().get_map_mut().translate(subtraction);
            stencil_vec[1].lock().unwrap().get_map_mut().translate(addition);
            canvas.update_stencils_arc_mutex(&stencil_vec);
            canvas.draw();
            for _ in 0..100000000{
                let f: f32 = 1.1234251324124;
                let _ = f.sqrt();
            }
        }
    }

    #[test]
    fn test_side_generation(){
        let test_side = Side::new((0.,0.), (1.,1.));
        assert_eq!(test_side.angle, 0.7853981633974483);
        assert_eq!(test_side.slope, 1.0);
        assert_eq!(test_side.length, 1.4142135623730951);
    }
}