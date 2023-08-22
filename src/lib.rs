pub mod canvas;
pub mod  stencil;
pub mod debug_logger;
pub mod stencil_buffer;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::{Mutex, Arc}};

    use crate::{stencil::{Stencil, StencilMap}, canvas::{Tile, Color, Canvas, Point}, debug_logger::debug_log, stencil_buffer::StencilBuffer};

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
    fn test_overlap_single(){
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let addition = Point{x:1, y: 0};
        let subtraction = Point{x:-1, y:0};
        let mut test_stencil_map = StencilMap::new(Point{x:0, y:0}, HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]));
        let mut test_stencil_map2 = StencilMap::new(Point{x:8, y:0}, HashMap::from([(Point{x:9,y:0}, tile), (Point{x:8, y:0}, tile), (Point{x:9, y:1}, tile), (Point{x:8,y:1}, tile)]));
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

    #[test]
    fn test_overlap_stencil(){
        let addition = Point{x:1, y: 0};
        let subtraction = Point{x:-1, y:0};
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let mut TestStencil1 = TestStencil{stencilmap: StencilMap::new(Point{x:0, y:0}, HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]))};
        let mut TestStencil2 = TestStencil{stencilmap: StencilMap::new(Point{x:8, y:0}, HashMap::from([(Point{x:9,y:0}, tile), (Point{x:8, y:0}, tile), (Point{x:9, y:1}, tile), (Point{x:8,y:1}, tile)]))};
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
        let addition = Point{x:1, y: 0};
        let subtraction = Point{x:-1, y:0};
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let mut TestStencil1: Mutex<Box<dyn Stencil>> = Mutex::new(Box::new(TestStencil{stencilmap: StencilMap::new(Point{x:0, y:0}, HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]))}));
        let mut TestStencil2: Mutex<Box<dyn Stencil>> = Mutex::new(Box::new(TestStencil{stencilmap: StencilMap::new(Point{x:8, y:0}, HashMap::from([(Point{x:9,y:0}, tile), (Point{x:8, y:0}, tile), (Point{x:9, y:1}, tile), (Point{x:8,y:1}, tile)]))}));
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
        let addition = Point{x:1, y: 0};
        let subtraction = Point{x:-1, y:0};
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let mut TestStencil1: Arc<Mutex<Box<dyn Stencil>>> = Arc::new(Mutex::new(Box::new(TestStencil{stencilmap: StencilMap::new(Point{x:0, y:0}, HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]))})));
        let mut TestStencil2: Arc<Mutex<Box<dyn Stencil>>> = Arc::new(Mutex::new(Box::new(TestStencil{stencilmap: StencilMap::new(Point{x:8, y:0}, HashMap::from([(Point{x:9,y:0}, tile), (Point{x:8, y:0}, tile), (Point{x:9, y:1}, tile), (Point{x:8,y:1}, tile)]))})));
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

    // fn stencil_buffger_test_update<'a>(v: &'a Vec<Arc<Mutex<Box<dyn Stencil>>>>){
    //     let mut test_buffer: StencilBuffer<'a> = StencilBuffer::new();
    //     let mut testing = Vec::new();
    //     for value in v{
    //         testing.push(value);
    //         // test_buffer.push(value.clone().lock().unwrap().get_map_mut())
    //     }
        //stencilbuffer needs to take type T then have an additional function to convert to the stencilmaps

    // }

    #[test]
    fn stencil_buffer_test(){
        let tile = Tile::new(Color::new(0, 200, 200, true));
        let mut test_buffer: StencilBuffer<dyn TestTrait> = StencilBuffer::new();
        let test1 = TestTraitStruct{map: StencilMap::new(Point{x:0, y:0}, HashMap::from([(Point{x:0,y:0}, tile), (Point{x:1, y:0}, tile), (Point{x:0, y:1}, tile), (Point{x:1,y:1}, tile)]))};
        let test2 = TestTraitStruct{map: StencilMap::new(Point{x:8, y:0}, HashMap::from([(Point{x:9,y:0}, tile), (Point{x:8, y:0}, tile), (Point{x:9, y:1}, tile), (Point{x:8,y:1}, tile)]))};
        let test_vec: Vec<Box<dyn TestTrait>> = vec![Box::new(test1), Box::new(test2)];
        for mut test_struct in test_vec{
            // let tmp = *test_struct;
            // test_buffer.push(&mut *test_struct);
        }
        // let test_buffer = StencilBuffer::new();
    }
}