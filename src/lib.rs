pub mod canvas;
pub mod  stencil;

#[cfg(test)]
mod tests {
    use crate::{stencil::{Stencil, Translation, StencilMap}, canvas::{Tile, Color, Canvas, Point}};


    struct TestStencil{

    }

    impl Stencil for TestStencil{
        fn get_map(&self) -> crate::stencil::StencilMap {
            let tile = Tile::new(Color::new(0, 200, 200, true), 1);
            let origin = Point{x:0, y:0};
            return vec![Translation::new(Point{x:0,y:0}, origin, None, tile), Translation::new(Point{x:0,y:1}, origin, None, tile), Translation::new(Point{x:1,y:0}, origin, None, tile), Translation::new(Point{x:1,y:1}, origin, None, tile)]
            // return vec![(Transformation::new(None, (0,0)), tile), (Transformation::new(None, (0,1)), tile), (Transformation::new(None, (1,0)), tile), (Transformation::new(None, (1,1)), tile)];
        }
    }
    #[test]
    fn test_stencil_draw(){
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let test_stencil = TestStencil{};
        canvas.update(test_stencil.get_map());
        canvas.draw();
    }

    #[test]
    fn test_animation(){
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let tile = Tile::new(Color::new(0, 200, 200, true), 1);
        let mut origin = Point{x:0, y:0};
        let addition = Point{x:1, y: 0};
        let test_stencil_map: StencilMap = vec![Translation::new(Point{x:0,y:0}, origin, None, tile), Translation::new(Point{x:0,y:1}, origin, None, tile), Translation::new(Point{x:1,y:0}, origin, None, tile), Translation::new(Point{x:1,y:1}, origin, None, tile)];
        canvas.update(test_stencil_map);
        canvas.draw();
        for j in 0..100000000{
            let f: f32 = 1.1234251324124;
            f.sqrt();
        }
        for i in 1..5{
            let new_origin = origin + addition;
            let test_stencil_map: StencilMap = vec![Translation::new(Point{x:0,y:0}, new_origin, Some(origin), tile), Translation::new(Point{x:0,y:1}, new_origin, Some(origin), tile), Translation::new(Point{x:1,y:0}, new_origin, Some(origin), tile), Translation::new(Point{x:1,y:1}, new_origin, Some(origin), tile)];
            canvas.update(test_stencil_map);
            canvas.draw();
            for j in 0..100000000{
                let f: f32 = 1.1234251324124;
                f.sqrt();
            }
            origin = new_origin;
        }
    }
}